use pg_core::{DbContext, OrderBy, PaginatedResponse, impl_repository};
use sea_orm::{prelude::*, *};
use time::OffsetDateTime;

use crate::{
    Repository, Result,
    entity::{prelude::Subject as SubjectEntity, subject},
    table::{
        dto::PaginationInput,
        subject::dto::{CreateSubject, ListSubject, Subject, SubjectId, SubjectKind},
    },
};

impl_repository!(SubjectRepo, SubjectEntity, subject::Model);

/// ===============================
/// Service（对外能力）
/// ===============================

/// Subject service（基础 service，单表）
pub struct SubjectService {
    repo: SubjectRepo,
}

impl SubjectService {
    /// 创建 service
    pub fn new(ctx: DbContext) -> Self {
        Self {
            repo: SubjectRepo::new(ctx.clone()),
        }
    }

    /// 创建一个新的 Subject
    pub async fn create(&self, input: CreateSubject) -> Result<Subject> {
        let active = subject::ActiveModel {
            subject_type: Set(input.kind.to_string()),
            created_at: Set(OffsetDateTime::now_utc()),
            ..Default::default()
        };

        let model = self.repo.insert(active).await?;

        Ok(Self::from_model(model))
    }

    /// 根据 ID 获取 Subject
    pub async fn get(&self, id: SubjectId) -> Result<Option<Subject>> {
        let model = self.repo.find_by_id(id.0).await?;
        Ok(model.map(Self::from_model))
    }

    /// 判断 Subject 是否存在
    pub async fn exists(&self, id: SubjectId) -> Result<bool> {
        self.repo.exists_by_id(id.0).await
    }

    /// 查询 Subject（可选按类型过滤）
    pub async fn list(
        &self,
        input: ListSubject,
        pagination: Option<PaginationInput>,
    ) -> Result<PaginatedResponse<Subject>> {
        let mut condition = Condition::all();
        let mut has_condition = false;

        if let Some(kind) = input.kind {
            condition = condition.add(subject::Column::SubjectType.eq(kind.to_string()));
            has_condition = true;
        }

        let condition = if has_condition { Some(condition) } else { None };
        let order_by = OrderBy::desc(subject::Column::CreatedAt);
        let params = pagination.unwrap_or_default().to_params();

        let response = self
            .repo
            .find_paginated(condition, &params, Some(&order_by))
            .await?;
        Ok(response.map(Self::from_model))
    }

    /// ===============================
    /// 内部映射
    /// ===============================

    fn from_model(model: subject::Model) -> Subject {
        Subject {
            id: SubjectId(model.subject_id),
            kind: SubjectKind::from(model.subject_type),
            created_at: model.created_at,
        }
    }
}
