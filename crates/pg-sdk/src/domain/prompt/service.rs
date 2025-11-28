use std::sync::Arc;

use sea_orm::{prelude::*, *};
use time::OffsetDateTime;

use super::{
    dto::{CreatePromptRequest, ListPromptsOptions, UpdatePromptRequest},
    repo::PromptRepo,
};
use crate::{
    Repository, Result, SdkError,
    entity::{prelude::Prompt, prompt},
    validate_struct,
};

/// Prompt service for business logic
pub struct PromptService {
    repo: PromptRepo,
}

impl PromptService {
    /// Create a new service instance
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self {
            repo: PromptRepo::new(db.clone()),
        }
    }

    // 事务使用
    pub fn db(&self) -> &DatabaseConnection {
        self.repo.db()
    }

    /// Create a new prompt
    pub async fn create(&self, req: CreatePromptRequest) -> Result<prompt::Model> {
        // Validate request
        validate_struct(&req)?;

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
        validate_struct(&req)?;

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
            .exec(self.repo.db())
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
    pub async fn list(&self, options: ListPromptsOptions) -> Result<(Vec<prompt::Model>, u64)> {
        let page = options.page.unwrap_or(1);
        let page_size = options.page_size.unwrap_or(20);

        let mut query = Prompt::find();

        if options.only_active {
            query = query.filter(prompt::Column::IsActive.eq(true));
        }

        // Order by update_time descending
        query = query.order_by_desc(prompt::Column::UpdateTime);

        // Get total count
        let total = query.clone().count(self.repo.db()).await?;

        // Apply pagination
        let paginator = query.paginate(self.repo.db(), page_size);
        let prompts = paginator.fetch_page(page - 1).await?;

        Ok((prompts, total))
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

        Ok(Prompt::find()
            .filter(
                Condition::any()
                    .add(prompt::Column::Id.eq(parent_id))
                    .add(prompt::Column::ParentId.eq(parent_id)),
            )
            .order_by_asc(prompt::Column::Version)
            .all(self.repo.db())
            .await?)
    }

    /// Get the active (latest) version of a prompt by parent_id
    pub async fn get_active(&self, parent_id: i64) -> Result<Option<prompt::Model>> {
        Ok(Prompt::find()
            .filter(
                Condition::all()
                    .add(
                        Condition::any()
                            .add(prompt::Column::Id.eq(parent_id))
                            .add(prompt::Column::ParentId.eq(parent_id)),
                    )
                    .add(prompt::Column::IsActive.eq(true)),
            )
            .one(self.repo.db())
            .await?)
    }
}
