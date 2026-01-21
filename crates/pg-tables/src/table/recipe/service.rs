use pg_core::{DbContext, Error as PgError, impl_repository};
use sea_orm::{prelude::*, *};
use time::OffsetDateTime;

use crate::{
    Repository, Result,
    entity::recipe,
    table::{
        metric::dto::MetricId,
        recipe::dto::{CreateRecipe, QueryRecipe, Recipe},
    },
};

impl_repository!(RecipeRepo, recipe::Entity, recipe::Model);

/// Recipe service（基础 service，单表）
pub struct RecipeService {
    repo: RecipeRepo,
}

impl RecipeService {
    /// 创建 service
    pub fn new(ctx: DbContext) -> Self {
        Self {
            repo: RecipeRepo::new(ctx.clone()),
        }
    }

    /// 创建一个新的 Recipe
    pub async fn create(&self, input: CreateRecipe) -> Result<Recipe> {
        let now = OffsetDateTime::now_utc();

        let active = recipe::ActiveModel {
            metric_id: Set(input.metric_id),
            deps: Set(input.deps),
            calc_key: Set(input.calc_key),
            arg_map: Set(input.arg_map),
            expr: Set(input.expr),
            created_at: Set(now),
            ..Default::default()
        };

        let model = self.repo.insert(active).await?;
        Ok(Self::from_model(model))
    }

    /// 根据 ID 获取 Recipe
    pub async fn get(&self, id: i64) -> Result<Recipe> {
        let model = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| PgError::not_found("Recipe", id))?;
        Ok(Self::from_model(model))
    }

    /// 查询 Recipe（可选过滤）
    pub async fn list(&self, input: QueryRecipe) -> Result<Vec<Recipe>> {
        let mut condition = Condition::all();
        let mut has_condition = false;

        if let Some(calc_key) = input.calc_key {
            condition = condition.add(recipe::Column::CalcKey.eq(calc_key));
            has_condition = true;
        }

        let models = if has_condition {
            self.repo.find_with_filter(condition).await?
        } else {
            self.repo.find_all().await?
        };

        Ok(models.into_iter().map(Self::from_model).collect())
    }

    /// 根据 metric_id 获取 Recipe
    pub async fn get_by_metric_id(&self, metric_id: MetricId) -> Result<Option<Recipe>> {
        let model = self
            .repo
            .select_one(
                self.repo
                    .query_filtered(Condition::all().add(recipe::Column::MetricId.eq(metric_id.0))),
            )
            .await?;
        Ok(model.map(Self::from_model))
    }

    fn from_model(model: recipe::Model) -> Recipe {
        Recipe {
            id: model.recipe_id,
            metric_id: model.metric_id,
            deps: model.deps,
            calc_key: model.calc_key,
            arg_map: model.arg_map,
            expr: model.expr,
            created_at: model.created_at,
        }
    }
}
