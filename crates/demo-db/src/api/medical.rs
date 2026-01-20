use pg_tables::{
    pg_core::DbContext,
    table::{
        data_source::{dto::DataSourceId, service::DataSourceService},
        observation::{
            dto::{ObservationQueryKey, RecordObservation},
            service::ObservationService,
        },
        recipe::service::RecipeService,
        subject::service::SubjectService,
    },
};
use time::OffsetDateTime;

use crate::{
    Error, Result,
    dto::{
        base::Range,
        medical::{
            ObservationQueryResult, QueryObservationSeries, RecordObservationRequest,
            RecordObservationResult, RecordObservationWithSourceRequest,
        },
    },
};

pub struct HealthApi {
    subject: SubjectService,
    recipe: RecipeService,
    observation: ObservationService,
    data_source: DataSourceService,
}

impl HealthApi {
    pub fn new(db: DbContext) -> Self {
        Self {
            subject: SubjectService::new(db.clone()),
            recipe: RecipeService::new(db.clone()),
            observation: ObservationService::new(db.clone()),
            data_source: DataSourceService::new(db),
        }
    }
}

impl HealthApi {
    pub async fn record_observation(&self, req: RecordObservationRequest) -> Result<()> {
        // 1. subject 必须存在
        self.subject
            .exists(req.subject_id)
            .await?
            .then(|| ())
            .ok_or_else(|| Error::not_found("subject", req.subject_id.0))?;

        // 2. recipe 必须存在
        self.recipe.get(req.recipe_id.0).await?;

        // 3. 组装 pg-tables 的 RecordObservation DTO
        let input = RecordObservation {
            subject_id: req.subject_id,
            recipe_id: req.recipe_id,
            value: req.value,
            observed_at: req.observed_at,
            source_id: req.source.map(|_| DataSourceId(0)), // demo：真实系统这里应做 source 映射
        };

        // 4. 调用真实的单表 service
        self.observation.record(input).await?;

        Ok(())
    }

    /// 记录观测数据（带 source 创建）
    /// 1. 先创建 data_source
    /// 2. 再插入 observation
    pub async fn record_observation_with_source(
        &self,
        req: RecordObservationWithSourceRequest,
    ) -> Result<RecordObservationResult> {
        // 1. subject 必须存在
        self.subject
            .exists(req.subject_id)
            .await?
            .then(|| ())
            .ok_or_else(|| Error::not_found("subject", req.subject_id.0))?;

        // 2. recipe 必须存在
        self.recipe.get(req.recipe_id.0).await?;

        // 3. 创建 data_source
        let data_source = self.data_source.create(req.source).await?;

        // 4. 插入 observation
        let input = RecordObservation {
            subject_id: req.subject_id,
            recipe_id: req.recipe_id,
            value: req.value,
            observed_at: req.observed_at,
            source_id: Some(data_source.id),
        };
        let observation = self.observation.record(input).await?;

        Ok(RecordObservationResult {
            observation_id: observation.id,
            source_id: data_source.id,
        })
    }

    pub async fn query_observation(
        &self,
        query: QueryObservationSeries,
        range: Range<OffsetDateTime>,
    ) -> Result<ObservationQueryResult> {
        let recipe = self.recipe.get(query.recipe_id.0).await?;

        let key = ObservationQueryKey {
            subject_id: query.subject_id.into(),
            recipe_id: query.recipe_id.into(),
        };

        let points = self
            .observation
            .query_observation(key, range.into())
            .await?;

        Ok(ObservationQueryResult { recipe, points })
    }

    pub async fn list_selectable_recipes(&self) -> Result<Vec<crate::dto::recipe::RecipeResponse>> {
        let req = pg_tables::table::recipe::dto::QueryRecipe {
            kind: Some(pg_tables::table::recipe::dto::RecipeKind::Derived),
            calc_key: None,
        };
        self.recipe.list(req).await
    }
}
