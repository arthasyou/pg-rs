use pg_core::{DbContext, Error, OrderBy, PaginatedResponse, impl_repository};
use sea_orm::{prelude::*, *};
use time::OffsetDateTime;

use crate::{
    Repository, Result,
    entity::{metric, prelude::Metric as MetricEntity},
    table::{
        dto::PaginationInput,
        metric::dto::{
            CreateMetric, ListMetric, Metric, MetricCode, MetricId, MetricValueType,
            MetricVisualization,
        },
    },
};

impl_repository!(MetricRepo, MetricEntity, metric::Model);

/// ===============================
/// Service（对外能力）
/// ===============================

/// Metric service（基础 service，单表）
pub struct MetricService {
    repo: MetricRepo,
}

impl MetricService {
    /// 创建 service
    pub fn new(ctx: DbContext) -> Self {
        Self {
            repo: MetricRepo::new(ctx.clone()),
        }
    }

    /// 创建一个新的 Metric
    pub async fn create(&self, input: CreateMetric) -> Result<Metric> {
        if self.exists_by_code(&input.code).await? {
            return Err(Error::already_exists("Metric", "metric_code", input.code.0));
        }

        let now = Self::now_utc();
        let active = metric::ActiveModel {
            metric_code: Set(input.code.0),
            metric_name: Set(input.name),
            unit: Set(input.unit),
            value_type: Set(input.value_type.to_string()),
            created_at: Set(now),
            ..Default::default()
        };

        let model = self.repo.insert(active).await?;
        Ok(Self::from_model(model))
    }

    /// 根据 ID 获取 Metric
    pub async fn get(&self, id: MetricId) -> Result<Option<Metric>> {
        let model = self.repo.find_by_id(id.0).await?;
        Ok(model.map(Self::from_model))
    }

    /// 根据 code 获取 Metric
    pub async fn get_by_code(&self, code: &MetricCode) -> Result<Option<Metric>> {
        let model = self
            .repo
            .select_one(
                self.repo
                    .query_filtered(Condition::all().add(metric::Column::MetricCode.eq(&code.0))),
            )
            .await?;
        Ok(model.map(Self::from_model))
    }

    /// 判断 Metric 是否存在
    pub async fn exists(&self, id: MetricId) -> Result<bool> {
        self.repo.exists_by_id(id.0).await
    }

    /// 判断 Metric code 是否存在
    pub async fn exists_by_code(&self, code: &MetricCode) -> Result<bool> {
        let found = self
            .repo
            .select_one(
                self.repo
                    .query_filtered(Condition::all().add(metric::Column::MetricCode.eq(&code.0))),
            )
            .await?;
        Ok(found.is_some())
    }

    /// 查询 Metric（可选按类型过滤）
    pub async fn list(
        &self,
        input: ListMetric,
        pagination: Option<PaginationInput>,
    ) -> Result<PaginatedResponse<Metric>> {
        let mut condition = Condition::all();
        let mut has_condition = false;

        if let Some(value_type) = input.value_type {
            condition = condition.add(metric::Column::ValueType.eq(value_type.to_string()));
            has_condition = true;
        }

        let condition = if has_condition { Some(condition) } else { None };
        let order_by = OrderBy::desc(metric::Column::CreatedAt);
        let params = pagination.unwrap_or_default().to_params();

        let response = self
            .repo
            .find_paginated(condition, &params, Some(&order_by))
            .await?;
        Ok(response.map(Self::from_model))
    }

    /// 获取可用于选择的 Metric 列表（给前端下拉框用）
    pub async fn list_selectable(&self) -> Result<Vec<Metric>> {
        let order_by = OrderBy::asc(metric::Column::MetricName);

        let condition = Condition::all();
        let models = self
            .repo
            .find_with_filter_and_order(condition, &order_by)
            .await?;

        Ok(models.into_iter().map(Self::from_model).collect())
    }

    fn now_utc() -> OffsetDateTime {
        OffsetDateTime::now_utc()
    }

    /// ===============================
    /// 内部映射
    /// ===============================

    fn from_model(model: metric::Model) -> Metric {
        Metric {
            id: MetricId(model.metric_id),
            code: MetricCode(model.metric_code),
            name: model.metric_name,
            unit: model.unit,
            value_type: MetricValueType::from(model.value_type),
            visualization: MetricVisualization::from(model.visualization),
            created_at: model.created_at,
        }
    }
}
