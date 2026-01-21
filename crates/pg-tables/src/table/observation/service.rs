use pg_core::{DbContext, OrderBy, impl_repository, query::SelectExt};
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
            Observation, ObservationId, ObservationInputs, ObservationPoint, ObservationQueryKey,
            ObservationValue, RecordObservation,
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

    pub async fn query_observation_by_metrics(
        &self,
        subject_id: SubjectId,
        metric_ids: Vec<MetricId>,
        range: Range<OffsetDateTime>,
    ) -> Result<Vec<ObservationInputs>> {
        if metric_ids.is_empty() {
            return Ok(Vec::new());
        }

        let metric_ids: Vec<i64> = metric_ids.into_iter().map(|id| id.0).collect();

        let mut condition = Condition::all()
            .add(observation::Column::SubjectId.eq(subject_id.0))
            .add(observation::Column::MetricId.is_in(metric_ids.clone()));

        if let Some(from) = range.from {
            condition = condition.add(observation::Column::ObservedAt.gte(from));
        }
        if let Some(to) = range.to {
            condition = condition.add(observation::Column::ObservedAt.lte(to));
        }

        let order_by = OrderBy::asc(observation::Column::ObservedAt);

        let mut query = ObservationEntity::find()
            .select_only()
            .column(observation::Column::ObservedAt);

        for id in &metric_ids {
            let expr = Expr::expr(
                Expr::case(
                    Expr::col(observation::Column::MetricId).eq(*id),
                    Expr::col(observation::Column::Value),
                )
                .finally(Expr::null()),
            )
            .max();
            query = query.column_as(expr, format!("metric_{id}"));
        }

        query = query
            .apply_condition(Some(condition))
            .apply_group_by(vec![observation::Column::ObservedAt])
            .apply_order(&order_by);

        let rows = self.repo.db().query_all(query.as_query()).await?;

        let mut observations = Vec::with_capacity(rows.len());
        for row in rows {
            let observed_at: OffsetDateTime = row.try_get("", "observed_at")?;
            let mut inputs = serde_json::Map::new();

            for id in &metric_ids {
                let col = format!("metric_{id}");
                let value: Option<String> = row.try_get("", &col)?;
                if let Some(value) = value {
                    inputs.insert(id.to_string(), serde_json::Value::String(value));
                }
            }

            observations.push(ObservationInputs {
                observed_at,
                inputs: serde_json::Value::Object(inputs),
            });
        }

        Ok(observations)
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
