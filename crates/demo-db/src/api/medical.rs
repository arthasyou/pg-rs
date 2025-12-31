use pg_tables::{
    pg_core::DbContext,
    table::{
        data_source::{dto::DataSourceId, service::DataSourceService},
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
    Error, Result,
    dto::{
        base::Range,
        medical::{
            ObservationQueryResult, QueryObservationSeries, RecordObservationRequest,
            RecordObservationWithSourceRequest, RecordObservationResult,
        },
    },
};

pub struct HealthApi {
    subject: SubjectService,
    metric: MetricService,
    observation: ObservationService,
    data_source: DataSourceService,
}

impl HealthApi {
    pub fn new(db: DbContext) -> Self {
        Self {
            subject: SubjectService::new(db.clone()),
            metric: MetricService::new(db.clone()),
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

        // 2. metric 必须存在
        self.metric
            .get(req.metric_id)
            .await?
            .ok_or_else(|| Error::not_found("metric", req.metric_id.0))?;

        // 3. 组装 pg-tables 的 RecordObservation DTO
        let input = RecordObservation {
            subject_id: req.subject_id,
            metric_id: req.metric_id,
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

        // 2. metric 必须存在
        self.metric
            .get(req.metric_id)
            .await?
            .ok_or_else(|| Error::not_found("metric", req.metric_id.0))?;

        // 3. 创建 data_source
        let data_source = self.data_source.create(req.source).await?;

        // 4. 插入 observation
        let input = RecordObservation {
            subject_id: req.subject_id,
            metric_id: req.metric_id,
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
        range: Range<PrimitiveDateTime>,
    ) -> Result<ObservationQueryResult> {
        let metric = self
            .metric
            .get(query.metric_id.into())
            .await?
            .ok_or(Error::db_not_found("metric"))?;

        let key = ObservationQueryKey {
            subject_id: query.subject_id.into(),
            metric_id: query.metric_id.into(),
        };

        let points = self
            .observation
            .query_observation(key, range.into())
            .await?;

        Ok(ObservationQueryResult { metric, points })
    }
}
