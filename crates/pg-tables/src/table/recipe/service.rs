use pg_core::{DbContext, Error as PgError, impl_repository};
use sea_orm::{prelude::*, *};
use time::OffsetDateTime;

use crate::{
    Repository, Result,
    entity::recipe,
    table::{
        metric::dto::{MetricCode, MetricStatus, MetricValueType, MetricVisualization},
        recipe::dto::{CreateRecipe, QueryRecipe, Recipe, RecipeKind},
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
            kind: Set(input.kind.to_string()),
            deps: Set(input.deps),
            calc_key: Set(input.calc_key),
            arg_map: Set(input.arg_map),
            expr: Set(input.expr),
            metric_code: Set(input.metric_code.0),
            metric_name: Set(input.metric_name),
            unit: Set(input.unit),
            value_type: Set(input.value_type.to_string()),
            visualization: Set(input.visualization.to_string()),
            status: Set(input.status.to_string()),
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

        if let Some(kind) = input.kind {
            condition = condition.add(recipe::Column::Kind.eq(kind.to_string()));
            has_condition = true;
        }
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

    fn from_model(model: recipe::Model) -> Recipe {
        Recipe {
            id: model.recipe_id,
            kind: RecipeKind::from(model.kind),
            deps: model.deps,
            calc_key: model.calc_key,
            arg_map: model.arg_map,
            expr: model.expr,
            metric_code: MetricCode(model.metric_code),
            metric_name: model.metric_name,
            unit: model.unit,
            value_type: MetricValueType::from(model.value_type),
            visualization: MetricVisualization::from(model.visualization),
            status: model.status.into(),
            created_at: model.created_at,
        }
    }
}
