use pg_core::{DbContext, OrderBy, impl_repository};
use sea_orm::{prelude::*, *};
use time::OffsetDateTime;

use crate::{
    Repository, Result,
    entity::{observation, prelude::Observation as ObservationEntity},
    table::{
        data_source::dto::DataSourceId,
        dto::Range,
        observation::dto::{
            Observation, ObservationId, ObservationPoint, ObservationQueryKey, ObservationValue,
            RecordObservation,
        },
        recipe::dto::RecipeId,
        subject::dto::SubjectId,
    },
};

impl_repository!(ObservationRepo, ObservationEntity, observation::Model);
/// ===============================
/// Service（对外能力）
/// ===============================

/// Observation service（核心事实写入 / 查询）
pub struct ObservationService {
    repo: ObservationRepo,
}

impl ObservationService {
    /// 创建 service
    pub fn new(ctx: DbContext) -> Self {
        Self {
            repo: ObservationRepo::new(ctx.clone()),
        }
    }

    /// 记录一次新的 Observation（事实写入）
    pub async fn record(&self, input: RecordObservation) -> Result<Observation> {
        let recorded_at = Self::now_utc();
        let active = observation::ActiveModel {
            subject_id: Set(input.subject_id.0),
            recipe_id: Set(input.recipe_id.0),
            value: Set(input.value.0),
            observed_at: Set(input.observed_at),
            recorded_at: Set(recorded_at),
            source_id: Set(input.source_id.map(|id| id.0)),
            ..Default::default()
        };

        let model = self.repo.insert(active).await?;
        Ok(Self::from_model(model))
    }

    /// 根据 ID 获取 Observation
    pub async fn get(&self, id: ObservationId) -> Result<Option<Observation>> {
        let model = self.repo.find_by_id(id.0).await?;
        Ok(model.map(Self::from_model))
    }

    pub async fn query_observation(
        &self,
        key: ObservationQueryKey,
        range: Range<OffsetDateTime>,
    ) -> Result<Vec<ObservationPoint>> {
        let mut condition = Condition::all()
            .add(observation::Column::SubjectId.eq(key.subject_id.0))
            .add(observation::Column::RecipeId.eq(key.recipe_id.0));

        if let Some(from) = range.from {
            condition = condition.add(observation::Column::ObservedAt.gte(from));
        }
        if let Some(to) = range.to {
            condition = condition.add(observation::Column::ObservedAt.lte(to));
        }

        let order_by = OrderBy::asc(observation::Column::ObservedAt);

        let observations = self
            .repo
            .find_with_filter_and_order(condition, &order_by)
            .await?;

        Ok(observations
            .into_iter()
            .map(|obs| ObservationPoint {
                value: obs.value.into(),
                observed_at: obs.observed_at,
            })
            .collect())
    }

    fn now_utc() -> OffsetDateTime {
        OffsetDateTime::now_utc()
    }

    /// ===============================
    /// 内部映射
    /// ===============================

    fn from_model(model: observation::Model) -> Observation {
        Observation {
            id: ObservationId(model.observation_id),
            subject_id: SubjectId(model.subject_id),
            recipe_id: RecipeId(model.recipe_id),
            value: ObservationValue(model.value),
            observed_at: model.observed_at,
            recorded_at: model.recorded_at,
            source_id: model.source_id.map(DataSourceId),
        }
    }
}
