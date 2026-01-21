use pg_tables::{
    pg_core::DbContext,
    table::{
        data_source::{dto::DataSourceId, service::DataSourceService},
        metric::{dto::Metric, service::MetricService},
        observation::{
            dto::{ObservationPoint, ObservationQueryKey, ObservationValue, RecordObservation},
            service::ObservationService,
        },
        recipe::{dto::RecipeKind, service::RecipeService},
        subject::service::SubjectService,
    },
};
use time::OffsetDateTime;

use crate::{
    Error, Result,
    calc::{get_calc, parse_inputs},
    dto::{
        base::Range,
        medical::{
            ObservationQueryResult, QueryObservationSeries, QueryRecipeObservationRequest,
            QueryRecipeObservationResponse, RecordObservationRequest, RecordObservationResult,
            RecordObservationWithSourceRequest,
        },
    },
};

pub struct HealthApi {
    subject: SubjectService,
    metric: MetricService,
    observation: ObservationService,
    data_source: DataSourceService,
    recipe: RecipeService,
}

impl HealthApi {
    pub fn new(db: DbContext) -> Self {
        Self {
            subject: SubjectService::new(db.clone()),
            metric: MetricService::new(db.clone()),
            observation: ObservationService::new(db.clone()),
            data_source: DataSourceService::new(db.clone()),
            recipe: RecipeService::new(db),
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
        range: Range<OffsetDateTime>,
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

    pub async fn list_selectable_metrics(&self) -> Result<Vec<Metric>> {
        self.metric.list_selectable().await
    }

    pub async fn query_composite_metric(
        &self,
        req: QueryRecipeObservationRequest,
        range: Range<OffsetDateTime>,
    ) -> Result<QueryRecipeObservationResponse> {
        let recipe = self.recipe.get(req.recipe_id.0).await?;

        let deps_raw: Vec<i64> = serde_json::from_value(recipe.deps.clone())
            .map_err(|_| Error::internal("invalid deps for recipe"))?;
        let deps = deps_raw
            .into_iter()
            .map(pg_tables::table::metric::dto::MetricId)
            .collect::<Vec<_>>();

        match recipe.kind.clone() {
            RecipeKind::Primitive => {
                let metric_id = *deps
                    .first()
                    .ok_or(Error::internal("missing deps for primitive recipe"))?;
                if deps.len() != 1 {
                    return Err(Error::internal(
                        "primitive recipe should have exactly one dep",
                    ));
                }

                let key = ObservationQueryKey {
                    subject_id: req.subject_id,
                    metric_id,
                };

                let points = self
                    .observation
                    .query_observation(key, range.into())
                    .await?;

                let metric = recipe.into();
                Ok(QueryRecipeObservationResponse { metric, points })
            }
            RecipeKind::Derived => {
                let calc_key = recipe
                    .calc_key
                    .as_deref()
                    .ok_or(Error::internal("missing calc_key"))?;
                let calc = get_calc(calc_key).ok_or(Error::internal("unknown calc_key"))?;

                let rows = self
                    .observation
                    .query_observation_by_metrics(req.subject_id, deps, range.into())
                    .await?;

                let mut points = Vec::with_capacity(rows.len());
                for row in rows {
                    let inputs = parse_inputs(&row.inputs)?;
                    let value = calc(&inputs)?;
                    points.push(ObservationPoint {
                        value: ObservationValue(value.to_string()),
                        observed_at: row.observed_at,
                    });
                }

                let metric = recipe.into();

                Ok(QueryRecipeObservationResponse { metric, points })
            }
        }
    }
}
