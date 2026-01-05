use pg_core::{DbContext, OrderBy, impl_repository};
use sea_orm::{prelude::*, *};
use time::OffsetDateTime;

use crate::{
    Repository, Result,
    entity::{observation, prelude::Observation as ObservationEntity},
    table::{
        data_source::dto::DataSourceId,
        dto::Range,
        metric::dto::MetricId,
        observation::dto::{
            Observation, ObservationId, ObservationPoint, ObservationQueryKey, ObservationValue,
            RecordObservation,
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

    pub async fn query_observation(
        &self,
        key: ObservationQueryKey,
        range: Range<OffsetDateTime>,
    ) -> Result<Vec<ObservationPoint>> {
        let mut condition = Condition::all()
            .add(observation::Column::SubjectId.eq(key.subject_id.0))
            .add(observation::Column::MetricId.eq(key.metric_id.0));

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
            metric_id: MetricId(model.metric_id),
            value: ObservationValue(model.value),
            observed_at: model.observed_at,
            recorded_at: model.recorded_at,
            source_id: model.source_id.map(DataSourceId),
        }
    }
}

// impl ObservationService {
//     /// 业务 JOIN 查询：Observation + Metric 定义
//     pub async fn query_with_metric(
//         &self,
//         subject_id: SubjectId,
//         metric_id: Option<MetricId>,
//         from: Option<PrimitiveDateTime>,
//         to: Option<PrimitiveDateTime>,
//         pagination: PaginationParams,
//     ) -> Result<PaginatedResponse<ObservationWithMetric>> {
//         // 1. 构造基础 Condition（只放“必须存在”的条件）
//         let condition = Condition::all().add(observation::Column::SubjectId.eq(subject_id.0));

//         // 2. 构造查询结构（只关心 JOIN / SELECT）
//         let base_query = ObservationEntity::find()
//             .join(JoinType::InnerJoin, observation::Relation::Metric.def())
//             .apply_condition(Some(condition))
//             // 可选条件
//             .apply_optional_eq(observation::Column::MetricId, metric_id.map(|v| v.0))
//             // 时间范围
//             .apply_time_range(observation::Column::ObservedAt, from, to);

//         // 3. 查询分页数据
//         let list = base_query
//             .clone()
//             .select_only()
//             // observation
//             .column_as(observation::Column::ObservationId, "observation_id")
//             .column_as(observation::Column::SubjectId, "subject_id")
//             .column_as(observation::Column::Value, "value")
//             .column_as(observation::Column::ObservedAt, "observed_at")
//             .column_as(observation::Column::SourceId, "source_id")
//             // metric
//             .column_as(metric::Column::MetricId, "metric_id")
//             .column_as(metric::Column::MetricCode, "metric_code")
//             .column_as(metric::Column::MetricName, "metric_name")
//             .column_as(metric::Column::Unit, "metric_unit")
//             .column_as(metric::Column::ValueType, "metric_value_type")
//             .column_as(metric::Column::Status, "metric_status")
//             .column_as(metric::Column::CreatedAt, "metric_created_at")
//             .pagination(&pagination)
//             .into_model::<ObservationWithMetricRow>()
//             .all(self.repo.db())
//             .await?;

//         // 4. 查询总数
//         let total = base_query.total_count(self.repo.db()).await;

//         Ok(PaginatedResponse::new(
//             list.into_iter().map(ObservationWithMetric::from).collect(),
//             &pagination,
//             total,
//         ))
//     }
// }
