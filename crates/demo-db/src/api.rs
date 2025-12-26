use pg_tables::{
    pg_core::DbContext,
    table::{
        data_source::dto::DataSourceId,
        metric::{dto::MetricId, service::MetricService},
        observation::{
            dto::{ObservationValue, RecordObservation},
            service::ObservationService,
        },
        subject::{dto::SubjectId, service::SubjectService},
    },
};

use crate::{
    dto::RecordObservationRequest,
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
            .exists(SubjectId(req.subject_id))
            .await
            .map_err(|_| DemoDbError::NotFound("subject"))?
        {
            return Err(DemoDbError::NotFound("subject"));
        }

        // 2. metric 必须存在
        if !self
            .metric
            .exists(MetricId(req.metric_id))
            .await
            .map_err(|_| DemoDbError::NotFound("metric"))?
        {
            return Err(DemoDbError::NotFound("metric"));
        }

        // 3. 组装 pg-tables 的 RecordObservation DTO
        let input = RecordObservation {
            subject_id: SubjectId(req.subject_id),
            metric_id: MetricId(req.metric_id),
            value: ObservationValue(req.value.to_string()),
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
}
