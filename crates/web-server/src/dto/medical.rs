use demo_db::{
    dto::{
        base::Range,
        medical::{ObservationQueryResult, QueryObservationSeries},
    },
    CreateDataSource, DataSourceKind, MetricId, ObservationValue, SubjectId,
};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::{OffsetDateTime, PrimitiveDateTime};
use utoipa::{IntoParams, ToSchema};

use crate::error::{Error, Result};

fn parse_pg_timestamp6(input: &str) -> Result<PrimitiveDateTime> {
    use time::macros::format_description;

    let input = input.trim();
    let fmt_no_fraction = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    if let Ok(dt) = PrimitiveDateTime::parse(input, &fmt_no_fraction) {
        return Ok(dt);
    }

    if let Some((_, fraction)) = input.rsplit_once('.') {
        if fraction.is_empty()
            || fraction.len() > 6
            || !fraction.as_bytes().iter().all(|b| b.is_ascii_digit())
        {
            return Err(Error::Custom(
                "invalid datetime format, expected 'YYYY-MM-DD HH:MM:SS[.ffffff]'".into(),
            ));
        }
    }

    let fmt_fraction = format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:1+]"
    );
    PrimitiveDateTime::parse(input, &fmt_fraction).map_err(|_| {
        Error::Custom("invalid datetime format, expected 'YYYY-MM-DD HH:MM:SS[.ffffff]'".into())
    })
}

fn format_pg_timestamp6(dt: PrimitiveDateTime) -> Result<String> {
    use time::macros::format_description;

    let fmt = format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]");
    dt.format(&fmt)
        .map_err(|_| Error::Custom("failed formatting datetime".into()))
}

// =========================
// Query Observation
// =========================

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct QueryObservationRequest {
    /// subject 全局 ID（web 层用裸 i64）
    pub subject_id: i64,

    /// metric 全局 ID
    pub metric_id: i64,

    /// 查询起始时间（Postgres timestamp(6): YYYY-MM-DD HH:MM:SS[.ffffff]）
    pub start_at: Option<String>,

    /// 查询结束时间（Postgres timestamp(6): YYYY-MM-DD HH:MM:SS[.ffffff]）
    pub end_at: Option<String>,
}

impl QueryObservationRequest {
    pub fn to_internal(self) -> Result<(QueryObservationSeries, Range<PrimitiveDateTime>)> {
        let start = match self.start_at {
            Some(ref s) => parse_pg_timestamp6(s)?,
            None => {
                let start = OffsetDateTime::from_unix_timestamp(0).unwrap();
                PrimitiveDateTime::new(start.date(), start.time())
            }
        };

        let end = match self.end_at {
            Some(ref s) => parse_pg_timestamp6(s)?,
            None => {
                let end = OffsetDateTime::now_utc();
                PrimitiveDateTime::new(end.date(), end.time())
            }
        };

        let range = Range {
            from: Some(start),
            to: Some(end),
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
                    observed_at: format_pg_timestamp6(p.observed_at)
                        .unwrap_or_else(|_| "invalid datetime".to_string()),
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
    /// Postgres timestamp(6) formatted string: YYYY-MM-DD HH:MM:SS.ffffff
    pub observed_at: String,
}

// =========================
// Record Observation
// =========================

#[derive(Debug, Deserialize, ToSchema)]
pub struct RecordObservationRequest {
    /// subject 全局 ID
    pub subject_id: i64,

    /// metric 全局 ID
    pub metric_id: i64,

    /// 观测值
    pub value: String,

    /// 观测发生的时间（Postgres timestamp(6): YYYY-MM-DD HH:MM:SS[.ffffff]）
    pub observed_at: String,

    /// 数据来源信息
    pub source: SourceInput,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SourceInput {
    /// 来源类型：device / manual / import / system
    pub kind: String,

    /// 来源名称
    pub name: String,

    /// 可选元数据
    pub metadata: Option<JsonValue>,
}

impl RecordObservationRequest {
    pub fn to_internal(
        self,
    ) -> Result<(SubjectId, MetricId, ObservationValue, PrimitiveDateTime, CreateDataSource)> {
        let observed_at = parse_pg_timestamp6(&self.observed_at)?;

        let source = CreateDataSource {
            kind: DataSourceKind::from(self.source.kind.as_str()),
            name: self.source.name,
            metadata: self.source.metadata,
        };

        Ok((
            SubjectId(self.subject_id),
            MetricId(self.metric_id),
            ObservationValue(self.value),
            observed_at,
            source,
        ))
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecordObservationResponse {
    pub observation_id: i64,
    pub source_id: i64,
}
