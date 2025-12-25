use std::sync::Arc;

use pg_core::{OrderBy, PaginatedResponse, impl_repository};
use sea_orm::{prelude::*, *};
use time::{OffsetDateTime, PrimitiveDateTime};

use crate::{
    Repository, Result,
    entity::{observation, prelude::Observation as ObservationEntity},
    table::{
        data_source::dto::DataSourceId,
        dto::PaginationInput,
        metric::dto::MetricId,
        observation::dto::{
            ListObservationByMetric, ListObservationBySubject, ListObservationByTimeRange,
            Observation, ObservationId, ObservationValue, RecordObservation,
        },
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
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self {
            repo: ObservationRepo::new(db),
        }
    }

    /// 记录一次新的 Observation（事实写入）
    pub async fn record(&self, input: RecordObservation) -> Result<Observation> {
        let recorded_at = Self::now_primitive();
        let active = observation::ActiveModel {
            subject_id: Set(input.subject_id.0),
            metric_id: Set(input.metric_id.0),
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

    /// 查询某个 Subject 的观测记录
    pub async fn list_by_subject(
        &self,
        input: ListObservationBySubject,
        pagination: Option<PaginationInput>,
    ) -> Result<PaginatedResponse<Observation>> {
        let condition = Condition::all().add(observation::Column::SubjectId.eq(input.subject_id.0));
        let order_by = OrderBy::desc(observation::Column::ObservedAt);
        let params = pagination.unwrap_or_default().to_params();

        let response = self
            .repo
            .find_paginated(Some(condition), &params, Some(&order_by))
            .await?;
        Ok(response.map(Self::from_model))
    }

    /// 查询某个 Metric 的观测记录
    pub async fn list_by_metric(
        &self,
        input: ListObservationByMetric,
        pagination: Option<PaginationInput>,
    ) -> Result<PaginatedResponse<Observation>> {
        let condition = Condition::all().add(observation::Column::MetricId.eq(input.metric_id.0));
        let order_by = OrderBy::desc(observation::Column::ObservedAt);
        let params = pagination.unwrap_or_default().to_params();

        let response = self
            .repo
            .find_paginated(Some(condition), &params, Some(&order_by))
            .await?;
        Ok(response.map(Self::from_model))
    }

    /// 按时间范围查询 Observation（可选 subject / metric）
    pub async fn list_by_time_range(
        &self,
        input: ListObservationByTimeRange,
        pagination: Option<PaginationInput>,
    ) -> Result<PaginatedResponse<Observation>> {
        let mut condition = Condition::all()
            .add(observation::Column::ObservedAt.gte(input.start))
            .add(observation::Column::ObservedAt.lte(input.end));

        if let Some(subject_id) = input.subject_id {
            condition = condition.add(observation::Column::SubjectId.eq(subject_id.0));
        }
        if let Some(metric_id) = input.metric_id {
            condition = condition.add(observation::Column::MetricId.eq(metric_id.0));
        }

        let order_by = OrderBy::desc(observation::Column::ObservedAt);
        let params = pagination.unwrap_or_default().to_params();

        let response = self
            .repo
            .find_paginated(Some(condition), &params, Some(&order_by))
            .await?;
        Ok(response.map(Self::from_model))
    }

    fn now_primitive() -> PrimitiveDateTime {
        let now_offset = OffsetDateTime::now_utc();
        PrimitiveDateTime::new(now_offset.date(), now_offset.time())
    }

    /// ===============================
    /// 内部映射
    /// ===============================

    fn from_model(model: observation::Model) -> Observation {
        Observation {
            id: ObservationId(model.observation_id),
            subject_id: SubjectId(model.subject_id),
            metric_id: MetricId(model.metric_id),
            value: ObservationValue(model.value),
            observed_at: model.observed_at,
            recorded_at: model.recorded_at,
            source_id: model.source_id.map(DataSourceId),
        }
    }
}
