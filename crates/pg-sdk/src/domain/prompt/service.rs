use std::sync::Arc;

use sea_orm::{prelude::*, *};
use time::OffsetDateTime;
use validator::Validate;

use super::{
    dto::{CreatePromptRequest, ListPromptsOptions, UpdatePromptRequest},
    repo::PromptRepo,
};
use crate::{
    PaginatedResponse, PaginationParams, Repository, Result, SdkError,
    entity::{prelude::Prompt, prompt},
};

/// Prompt service for business logic
pub struct PromptService {
    repo: PromptRepo,
}

impl PromptService {
    /// Create a new service instance
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self {
            repo: PromptRepo::new(db),
        }
    }

    /// Get database connection for transactions
    pub fn db(&self) -> &DatabaseConnection {
        self.repo.db()
    }

    /// Create a new prompt
    pub async fn create(&self, req: CreatePromptRequest) -> Result<prompt::Model> {
        // Validate request
        req.validate()?;

        let now = OffsetDateTime::now_utc();

        let prompt = prompt::ActiveModel {
            title: Set(req.title),
            content: Set(req.content),
            version: Set(1),
            parent_id: NotSet,
            is_active: Set(true),
            tags: Set(req.tags),
            create_time: Set(now),
            update_time: Set(now),
            ..Default::default()
        };

        // Use repository to insert
        let result = self.repo.insert(prompt).await?;

        // Update parent_id to point to itself (first version)
        let mut active_model: prompt::ActiveModel = result.clone().into();
        active_model.parent_id = Set(Some(result.id));
        self.repo.update(active_model).await
    }

    /// Update an existing prompt (creates a new version)
    pub async fn update(&self, id: i64, req: UpdatePromptRequest) -> Result<prompt::Model> {
        // Validate request
        req.validate()?;

        // Get the original prompt using repository
        let original = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| SdkError::not_found("Prompt", id))?;

        let parent_id = original.parent_id.unwrap_or(original.id);

        // Deactivate all previous versions of this prompt
        Prompt::update_many()
            .col_expr(prompt::Column::IsActive, Expr::value(false))
            .filter(
                Condition::any()
                    .add(prompt::Column::Id.eq(parent_id))
                    .add(prompt::Column::ParentId.eq(parent_id)),
            )
            .exec(self.db())
            .await?;

        // Create new version
        let now = OffsetDateTime::now_utc();
        let new_version = prompt::ActiveModel {
            title: Set(req.title.unwrap_or(original.title)),
            content: Set(req.content.unwrap_or(original.content)),
            version: Set(original.version + 1),
            parent_id: Set(Some(parent_id)),
            is_active: Set(true),
            tags: Set(req.tags.or(original.tags)),
            create_time: Set(now),
            update_time: Set(now),
            ..Default::default()
        };

        // Use repository to insert
        self.repo.insert(new_version).await
    }

    /// Delete a prompt (soft delete by setting is_active to false)
    pub async fn delete(&self, id: i64) -> Result<()> {
        // Get prompt using repository
        let prompt = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| SdkError::not_found("Prompt", id))?;

        let parent_id = prompt.parent_id.unwrap_or(prompt.id);

        // Deactivate all versions
        Prompt::update_many()
            .col_expr(prompt::Column::IsActive, Expr::value(false))
            .filter(
                Condition::any()
                    .add(prompt::Column::Id.eq(parent_id))
                    .add(prompt::Column::ParentId.eq(parent_id)),
            )
            .exec(self.db())
            .await?;

        Ok(())
    }

    /// Get a prompt by ID
    pub async fn get_by_id(&self, id: i64) -> Result<Option<prompt::Model>> {
        self.repo.find_by_id(id).await
    }

    /// List prompts with pagination
    pub async fn list(
        &self,
        options: ListPromptsOptions,
    ) -> Result<PaginatedResponse<prompt::Model>> {
        let params =
            PaginationParams::new(options.page.unwrap_or(1), options.page_size.unwrap_or(20));

        // Build query with filter and order
        let mut query = self.repo.query();

        if options.only_active {
            query = query.filter(prompt::Column::IsActive.eq(true));
        }

        query = query.order_by_desc(prompt::Column::UpdateTime);

        // Use repository's find_paginated
        self.repo.find_paginated(&params, query).await
    }

    /// Get all versions of a prompt
    pub async fn get_versions(&self, id: i64) -> Result<Vec<prompt::Model>> {
        // Get prompt using repository
        let prompt = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| SdkError::not_found("Prompt", id))?;

        let parent_id = prompt.parent_id.unwrap_or(prompt.id);

        // Use repository's query builder and select_all
        let query = self
            .repo
            .query_filtered(
                Condition::any()
                    .add(prompt::Column::Id.eq(parent_id))
                    .add(prompt::Column::ParentId.eq(parent_id)),
            )
            .order_by_asc(prompt::Column::Version);

        self.repo.select_all(query).await
    }

    /// Get the active (latest) version of a prompt by parent_id
    pub async fn get_active(&self, parent_id: i64) -> Result<Option<prompt::Model>> {
        let query = self.repo.query_filtered(
            Condition::all()
                .add(
                    Condition::any()
                        .add(prompt::Column::Id.eq(parent_id))
                        .add(prompt::Column::ParentId.eq(parent_id)),
                )
                .add(prompt::Column::IsActive.eq(true)),
        );

        self.repo.select_one(query).await
    }
}
