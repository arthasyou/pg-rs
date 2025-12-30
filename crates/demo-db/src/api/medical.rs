use pg_tables::{
    pg_core::DbContext,
    table::{
        data_source::dto::DataSourceId,
        metric::service::MetricService,
        observation::{
            dto::{ObservationQueryKey, RecordObservation},
            service::ObservationService,
        },
        subject::service::SubjectService,
    },
};
use time::PrimitiveDateTime;

use crate::{
    dto::{
        base::Range,
        medical::{ObservationQueryResult, QueryObservationSeries, RecordObservationRequest},
    },
    error::{DemoDbError, Result},
};

pub struct HealthApi {
    subject: SubjectService,
    metric: MetricService,
    observation: ObservationService,
}

impl HealthApi {
    pub fn new(db: DbContext) -> Self {
        Self {
            subject: SubjectService::new(db.clone()),
            metric: MetricService::new(db.clone()),
            observation: ObservationService::new(db),
        }
    }
}

impl HealthApi {
    pub async fn record_observation(&self, req: RecordObservationRequest) -> Result<()> {
        // 1. subject 必须存在
        if !self
            .subject
            .exists(req.subject_id)
            .await
            .map_err(|_| DemoDbError::NotFound("subject"))?
        {
            return Err(DemoDbError::NotFound("subject"));
        }

        // 2. metric 必须存在
        if !self
            .metric
            .exists(req.metric_id)
            .await
            .map_err(|_| DemoDbError::NotFound("metric"))?
        {
            return Err(DemoDbError::NotFound("metric"));
        }

        // 3. 组装 pg-tables 的 RecordObservation DTO
        let input = RecordObservation {
            subject_id: req.subject_id,
            metric_id: req.metric_id,
            value: req.value,
            observed_at: req.observed_at,
            source_id: req.source.map(|_| DataSourceId(0)), // demo：真实系统这里应做 source 映射
        };

        // 4. 调用真实的单表 service
        self.observation
            .record(input)
            .await
            .map_err(|_| DemoDbError::Internal)?;

        Ok(())
    }

    pub async fn query_observation(
        &self,
        query: QueryObservationSeries,
        range: Range<PrimitiveDateTime>,
    ) -> Result<ObservationQueryResult> {
        let metric = self
            .metric
            .get(query.metric_id.into())
            .await
            .map_err(|_| DemoDbError::NotFound("metric"))?
            .ok_or(DemoDbError::NotFound("metric"))?;

        let key = ObservationQueryKey {
            subject_id: query.subject_id.into(),
            metric_id: query.metric_id.into(),
        };

        let points = self
            .observation
            .query_observation(key, range.into())
            .await
            .map_err(|_| DemoDbError::NotFound("metric"))?;

        Ok(ObservationQueryResult { metric, points })
    }
}
