use std::sync::Arc;

use pg_core::{OrderBy, PaginatedResponse, impl_repository};
use sea_orm::{prelude::*, *};
use time::{OffsetDateTime, PrimitiveDateTime};

use crate::{
    Repository, Result,
    entity::{data_source, prelude::DataSource as DataSourceEntity},
    table::{
        data_source::dto::{
            CreateDataSource, DataSource, DataSourceId, DataSourceKind, ListDataSource,
        },
        dto::PaginationInput,
    },
};

impl_repository!(DataSourceRepo, DataSourceEntity, data_source::Model);

/// ===============================
/// Service（对外能力）
/// ===============================

/// DataSource service（基础 service，单表）
pub struct DataSourceService {
    repo: DataSourceRepo,
}

impl DataSourceService {
    /// 创建 service
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self {
            repo: DataSourceRepo::new(db),
        }
    }

    /// 创建一个新的 DataSource
    pub async fn create(&self, input: CreateDataSource) -> Result<DataSource> {
        let now = Self::now_primitive();

        let active = data_source::ActiveModel {
            source_type: Set(input.kind.to_string()),
            source_name: Set(input.name),
            metadata: Set(input.metadata),
            created_at: Set(now),
            ..Default::default()
        };

        let model = self.repo.insert(active).await?;
        Ok(Self::from_model(model))
    }

    /// 根据 ID 获取 DataSource
    pub async fn get(&self, id: DataSourceId) -> Result<Option<DataSource>> {
        let model = self.repo.find_by_id(id.0).await?;
        Ok(model.map(Self::from_model))
    }

    /// 判断 DataSource 是否存在
    pub async fn exists(&self, id: DataSourceId) -> Result<bool> {
        self.repo.exists_by_id(id.0).await
    }

    /// 查询 DataSource（可选按类型过滤）
    pub async fn list(
        &self,
        input: ListDataSource,
        pagination: Option<PaginationInput>,
    ) -> Result<PaginatedResponse<DataSource>> {
        let mut condition = Condition::all();
        let mut has_condition = false;

        if let Some(kind) = input.kind {
            condition = condition.add(data_source::Column::SourceType.eq(kind.to_string()));
            has_condition = true;
        }

        let condition = if has_condition { Some(condition) } else { None };
        let order_by = OrderBy::desc(data_source::Column::CreatedAt);
        let params = pagination.unwrap_or_default().to_params();

        let response = self
            .repo
            .find_paginated(condition, &params, Some(&order_by))
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

    fn from_model(model: data_source::Model) -> DataSource {
        DataSource {
            id: DataSourceId(model.source_id),
            kind: DataSourceKind::from(model.source_type),
            name: model.source_name,
            metadata: model.metadata,
            created_at: model.created_at,
        }
    }
}
