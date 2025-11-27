use sea_orm::{prelude::*, *};
use time::OffsetDateTime;
use validator::Validate;

use crate::{
    core::validate::validate_struct,
    entity::prompt::{self, Entity as Prompt},
    error::{Result, SdkError},
};

/// Create prompt request
#[derive(Debug, Clone, Validate)]
pub struct CreatePromptRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,

    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,

    #[validate(length(max = 500, message = "Tags must be at most 500 characters"))]
    pub tags: Option<String>,
}

/// Update prompt request
#[derive(Debug, Clone, Validate)]
pub struct UpdatePromptRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: Option<String>,

    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: Option<String>,

    #[validate(length(max = 500, message = "Tags must be at most 500 characters"))]
    pub tags: Option<String>,
}

/// List prompts query options
#[derive(Debug, Clone, Default)]
pub struct ListPromptsOptions {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub only_active: bool,
}

/// Create a new prompt
pub async fn create_prompt(
    db: &DatabaseConnection,
    req: CreatePromptRequest,
) -> Result<prompt::Model> {
    // Validate request
    validate_struct(&req).map_err(SdkError::validation)?;

    let now = OffsetDateTime::now_utc();

    let prompt = prompt::ActiveModel {
        title: Set(req.title),
        content: Set(req.content),
        version: Set(1),
        parent_id: NotSet, // Will be set to id after insert
        is_active: Set(true),
        tags: Set(req.tags),
        create_time: Set(now),
        update_time: Set(now),
        ..Default::default()
    };

    let result = prompt.insert(db).await?;

    // Update parent_id to point to itself (first version)
    let mut active_model: prompt::ActiveModel = result.clone().into();
    active_model.parent_id = Set(Some(result.id));
    Ok(active_model.update(db).await?)
}

/// Update an existing prompt (creates a new version)
pub async fn update_prompt(
    db: &DatabaseConnection,
    id: i64,
    req: UpdatePromptRequest,
) -> Result<prompt::Model> {
    // Validate request
    validate_struct(&req).map_err(SdkError::validation)?;

    // Get the original prompt
    let original = Prompt::find_by_id(id)
        .one(db)
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
        .exec(db)
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

    Ok(new_version.insert(db).await?)
}

/// Delete a prompt (soft delete by setting is_active to false)
pub async fn delete_prompt(db: &DatabaseConnection, id: i64) -> Result<()> {
    let prompt = Prompt::find_by_id(id)
        .one(db)
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
        .exec(db)
        .await?;

    Ok(())
}

/// Get a prompt by ID
pub async fn get_prompt_by_id(db: &DatabaseConnection, id: i64) -> Result<Option<prompt::Model>> {
    Ok(Prompt::find_by_id(id).one(db).await?)
}

/// List prompts with pagination
pub async fn list_prompts(
    db: &DatabaseConnection,
    options: ListPromptsOptions,
) -> Result<(Vec<prompt::Model>, u64)> {
    let page = options.page.unwrap_or(1);
    let page_size = options.page_size.unwrap_or(20);

    let mut query = Prompt::find();

    if options.only_active {
        query = query.filter(prompt::Column::IsActive.eq(true));
    }

    // Order by update_time descending
    query = query.order_by_desc(prompt::Column::UpdateTime);

    // Get total count
    let total = query.clone().count(db).await?;

    // Apply pagination
    let paginator = query.paginate(db, page_size);
    let prompts = paginator.fetch_page(page - 1).await?;

    Ok((prompts, total))
}

/// Get all versions of a prompt
pub async fn get_prompt_versions(db: &DatabaseConnection, id: i64) -> Result<Vec<prompt::Model>> {
    let prompt = Prompt::find_by_id(id)
        .one(db)
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
        .all(db)
        .await?)
}

/// Get the active (latest) version of a prompt by parent_id
pub async fn get_active_prompt(
    db: &DatabaseConnection,
    parent_id: i64,
) -> Result<Option<prompt::Model>> {
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
        .one(db)
        .await?)
}
