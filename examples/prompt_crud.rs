use std::sync::Arc;

use pg_core::{DatabaseConfig, DatabaseManager};
use pg_sdk::domain::prompt::{
    CreatePromptRequest, ListPromptsOptions, PromptService, UpdatePromptRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    println!("🚀 Starting Prompt CRUD Example");
    println!("================================\n");

    // Initialize database manager
    println!("📦 Connecting to database...");
    let config = DatabaseConfig::new("default", &database_url);
    let manager = DatabaseManager::new(vec![config]).await?;
    let db = Arc::new(manager.get("default")?.clone());
    println!("✅ Connected to database\n");

    // Create service
    let service = PromptService::new(db);

    // Test 1: Create prompts
    println!("📝 Test 1: Creating prompts");
    println!("----------------------------");

    let prompt1 = service
        .create(CreatePromptRequest {
            title: "Rust Expert".to_string(),
            content: "You are an expert Rust programmer. Help users write safe and efficient Rust \
                      code."
                .to_string(),
            tags: Some("rust,programming,expert".to_string()),
        })
        .await?;
    println!(
        "✅ Created prompt 1: id={}, title={}",
        prompt1.id, prompt1.title
    );

    let prompt2 = service
        .create(CreatePromptRequest {
            title: "Python Tutor".to_string(),
            content: "You are a patient Python tutor. Explain concepts clearly with examples."
                .to_string(),
            tags: Some("python,education,tutor".to_string()),
        })
        .await?;
    println!(
        "✅ Created prompt 2: id={}, title={}",
        prompt2.id, prompt2.title
    );

    let prompt3 = service
        .create(CreatePromptRequest {
            title: "Code Reviewer".to_string(),
            content: "You are a thorough code reviewer. Provide constructive feedback on code \
                      quality."
                .to_string(),
            tags: Some("review,quality,feedback".to_string()),
        })
        .await?;
    println!(
        "✅ Created prompt 3: id={}, title={}\n",
        prompt3.id, prompt3.title
    );

    // Test 2: Get by ID
    println!("🔍 Test 2: Getting prompt by ID");
    println!("----------------------------");
    let found = service.get_by_id(prompt1.id).await?;
    match found {
        Some(p) => println!(
            "✅ Found prompt: id={}, title={}, version={}\n",
            p.id, p.title, p.version
        ),
        None => println!("❌ Prompt not found\n"),
    }

    // Test 3: List all prompts
    println!("📋 Test 3: Listing all prompts");
    println!("----------------------------");
    let list_result = service
        .list(ListPromptsOptions {
            page: Some(1),
            page_size: Some(10),
            only_active: true,
        })
        .await?;

    println!(
        "✅ Found {} prompts (total: {})",
        list_result.items.len(),
        list_result.total
    );
    for prompt in &list_result.items {
        println!(
            "   - id={}, title={}, version={}, active={}",
            prompt.id, prompt.title, prompt.version, prompt.is_active
        );
    }
    println!();

    // Test 4: Update prompt (creates new version)
    println!("🔄 Test 4: Updating prompt (creating new version)");
    println!("----------------------------");
    let updated = service
        .update(
            prompt1.id,
            UpdatePromptRequest {
                title: Some("Senior Rust Expert".to_string()),
                content: Some(
                    "You are a senior Rust expert with 10+ years of experience. Help users write \
                     production-ready Rust code."
                        .to_string(),
                ),
                tags: Some("rust,programming,expert,senior".to_string()),
            },
        )
        .await?;
    println!(
        "✅ Updated prompt: id={}, title={}, version={}\n",
        updated.id, updated.title, updated.version
    );

    // Test 5: Get all versions
    println!("📚 Test 5: Getting all versions of a prompt");
    println!("----------------------------");
    let versions = service.get_versions(prompt1.id).await?;
    println!("✅ Found {} versions:", versions.len());
    for v in &versions {
        println!(
            "   - version={}, id={}, title={}, active={}",
            v.version, v.id, v.title, v.is_active
        );
    }
    println!();

    // Test 6: Get active version
    println!("⭐ Test 6: Getting active version");
    println!("----------------------------");
    let parent_id = prompt1.parent_id.unwrap_or(prompt1.id);
    let active = service.get_active(parent_id).await?;
    match active {
        Some(p) => println!(
            "✅ Active version: id={}, title={}, version={}\n",
            p.id, p.title, p.version
        ),
        None => println!("❌ No active version found\n"),
    }

    // Test 7: Pagination
    println!("📄 Test 7: Testing pagination");
    println!("----------------------------");
    let page1 = service
        .list(ListPromptsOptions {
            page: Some(1),
            page_size: Some(2),
            only_active: true,
        })
        .await?;
    println!(
        "✅ Page 1: {} items, total={}, total_pages={}",
        page1.items.len(),
        page1.total,
        page1.total_pages
    );

    let page2 = service
        .list(ListPromptsOptions {
            page: Some(2),
            page_size: Some(2),
            only_active: true,
        })
        .await?;
    println!(
        "✅ Page 2: {} items, total={}, total_pages={}\n",
        page2.items.len(),
        page2.total,
        page2.total_pages
    );

    // Test 8: Filter only active
    println!("🔍 Test 8: Filtering active prompts");
    println!("----------------------------");
    let all_prompts = service
        .list(ListPromptsOptions {
            page: Some(1),
            page_size: Some(100),
            only_active: false,
        })
        .await?;
    println!("✅ All prompts (including inactive): {}", all_prompts.total);

    let active_prompts = service
        .list(ListPromptsOptions {
            page: Some(1),
            page_size: Some(100),
            only_active: true,
        })
        .await?;
    println!("✅ Active prompts only: {}\n", active_prompts.total);

    // Test 9: Soft delete
    println!("🗑️  Test 9: Soft deleting prompt");
    println!("----------------------------");
    service.delete(prompt2.id).await?;
    println!("✅ Soft deleted prompt id={}", prompt2.id);

    let deleted = service.get_by_id(prompt2.id).await?;
    match deleted {
        Some(p) => println!("   - Prompt still exists but is_active={}\n", p.is_active),
        None => println!("   - Prompt not found\n"),
    }

    // Test 10: Validation
    println!("✅ Test 10: Testing validation");
    println!("----------------------------");
    let invalid_result = service
        .create(CreatePromptRequest {
            title: "".to_string(), // Invalid: empty title
            content: "Test content".to_string(),
            tags: None,
        })
        .await;

    match invalid_result {
        Ok(_) => println!("❌ Should have failed validation"),
        Err(e) => println!("✅ Validation correctly failed: {}\n", e),
    }

    println!("================================");
    println!("✨ All tests completed successfully!");

    Ok(())
}
