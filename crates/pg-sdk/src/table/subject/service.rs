use std::sync::Arc;

use pg_core::impl_repository;
use sea_orm::{prelude::*, *};
use time::{OffsetDateTime, PrimitiveDateTime};

use crate::{
    Repository, Result,
    entity::{prelude::Subject as SubjectEntity, subject},
    table::subject::dto::{Subject, SubjectId, SubjectKind},
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
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self {
            repo: SubjectRepo::new(db),
        }
    }

    /// 创建一个新的 Subject
    pub async fn create(&self, kind: SubjectKind) -> Result<Subject> {
        let now_offset = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now_offset.date(), now_offset.time());

        let active = subject::ActiveModel {
            subject_type: Set(kind.to_string()),
            created_at: Set(now),
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
