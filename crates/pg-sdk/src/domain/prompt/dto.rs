use validator::Validate;

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
