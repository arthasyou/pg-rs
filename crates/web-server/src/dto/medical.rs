use demo_db::dto::{
    base::Range,
    medical::{ObservationQueryResult, QueryObservationSeries},
};
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, PrimitiveDateTime};
use utoipa::{IntoParams, PartialSchema, ToSchema};

use crate::error::{Error, Result};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct QueryObservationRequest {
    /// subject 全局 ID（web 层用裸 i64）
    pub subject_id: i64,

    /// metric 全局 ID
    pub metric_id: i64,

    /// 查询起始时间（RFC 3339）
    pub start_at: Option<String>,

    /// 查询结束时间（RFC 3339）
    pub end_at: Option<String>,
}

impl QueryObservationRequest {
    pub fn to_internal(self) -> Result<(QueryObservationSeries, Range<PrimitiveDateTime>)> {
        use time::{OffsetDateTime, PrimitiveDateTime, format_description::well_known::Rfc3339};

        let start = match self.start_at {
            Some(ref s) => OffsetDateTime::parse(s, &Rfc3339)
                .map_err(|_| Error::Custom("invalid start_at format".into()))?,
            None => OffsetDateTime::from_unix_timestamp(0).unwrap(),
        };

        let end = match self.end_at {
            Some(ref s) => OffsetDateTime::parse(s, &Rfc3339)
                .map_err(|_| Error::Custom("invalid end_at format".into()))?,
            None => OffsetDateTime::now_utc(),
        };

        let range = Range {
            from: Some(PrimitiveDateTime::new(start.date(), start.time())),
            to: Some(PrimitiveDateTime::new(end.date(), end.time())),
        };

        let query = QueryObservationSeries {
            subject_id: self.subject_id.into(),
            metric_id: self.metric_id.into(),
        };

        Ok((query, range))
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QueryObservationResponse {
    pub subject_id: i64,
    pub metric: MetricDto,
    pub points: Vec<ObservationPointDto>,
}

impl From<(i64, ObservationQueryResult)> for QueryObservationResponse {
    fn from((subject_id, v): (i64, ObservationQueryResult)) -> Self {
        Self {
            subject_id,
            points: v
                .points
                .into_iter()
                .map(|p| ObservationPointDto {
                    value: p.value.as_str().to_string(),
                    value_num: v.metric.try_parse_numeric(&p.value),
                    observed_at: OffsetDateTime::from_unix_timestamp(
                        p.observed_at.assume_utc().unix_timestamp(),
                    )
                    .unwrap(),
                })
                .collect(),
            metric: MetricDto {
                id: v.metric.id.0,
                code: v.metric.code.as_ref().to_string(),
                name: v.metric.name,
                unit: v.metric.unit,
            },
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MetricDto {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub unit: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ObservationPointDto {
    pub value: String,
    pub value_num: Option<f64>,
    #[schema(schema_with = String::schema)]
    pub observed_at: OffsetDateTime,
}
