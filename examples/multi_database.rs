/// Multi-database example demonstrating how to manage multiple database connections
///
/// Run with: cargo run --example multi_database
///
/// This example shows:
/// - Connecting to multiple databases simultaneously
/// - Accessing different databases by name
/// - Managing connection lifecycles
use pg_core::{DatabaseConfig, DatabaseManager};

#[tokio::main]
async fn main() -> pg_core::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    println!("=== Multi-Database Connection Example ===\n");

    // Define multiple database configurations
    let configs = vec![
        DatabaseConfig::new(
            "main",
            std::env::var("MAIN_DATABASE_URL").unwrap_or_else(|_| {
                "postgres://postgres:postgres@localhost:5432/main_db".to_string()
            }),
        )
        .max_connections(20)
        .with_sql_logging(true),
        DatabaseConfig::new(
            "replica",
            std::env::var("REPLICA_DATABASE_URL").unwrap_or_else(|_| {
                "postgres://postgres:postgres@localhost:5432/replica_db".to_string()
            }),
        )
        .max_connections(10)
        .with_sql_logging(false),
        DatabaseConfig::new(
            "analytics",
            std::env::var("ANALYTICS_DATABASE_URL").unwrap_or_else(|_| {
                "postgres://postgres:postgres@localhost:5432/analytics_db".to_string()
            }),
        )
        .max_connections(5)
        .with_sql_logging(true),
    ];

    println!("1. Configured databases:");
    for config in &configs {
        println!(
            "   - {} (max_connections: {})",
            config.name, config.max_connections
        );
    }
    println!();

    println!("2. Connecting to all databases...");
    let manager = DatabaseManager::new(configs).await?;
    println!("   ✓ All connections established\n");

    println!("3. Database manager info:");
    println!("   - Total databases: {}", manager.count());
    println!("   - Available databases: {:?}\n", manager.list_databases());

    // Access different databases
    println!("4. Accessing individual databases:\n");

    // Main database
    println!("   a) Main database:");
    match manager.get("main") {
        Ok(db) => match db.ping().await {
            Ok(_) => println!("      ✓ Main database is reachable"),
            Err(e) => println!("      ✗ Main database ping failed: {}", e),
        },
        Err(e) => println!("      ✗ Failed to get main database: {}", e),
    }

    // Replica database
    println!("\n   b) Replica database:");
    match manager.get("replica") {
        Ok(db) => match db.ping().await {
            Ok(_) => println!("      ✓ Replica database is reachable"),
            Err(e) => println!("      ✗ Replica database ping failed: {}", e),
        },
        Err(e) => println!("      ✗ Failed to get replica database: {}", e),
    }

    // Analytics database
    println!("\n   c) Analytics database:");
    match manager.get("analytics") {
        Ok(db) => match db.ping().await {
            Ok(_) => println!("      ✓ Analytics database is reachable"),
            Err(e) => println!("      ✗ Analytics database ping failed: {}", e),
        },
        Err(e) => println!("      ✗ Failed to get analytics database: {}", e),
    }

    // Check for non-existent database
    println!("\n   d) Non-existent database:");
    match manager.get("nonexistent") {
        Ok(_) => println!("      ✗ This shouldn't happen!"),
        Err(e) => println!("      ✓ Expected error: {}", e),
    }

    // Use default database
    println!("\n5. Using default database:");
    match manager.default() {
        Ok(db) => match db.ping().await {
            Ok(_) => println!("   ✓ Default database is reachable"),
            Err(e) => println!("   ✗ Default database ping failed: {}", e),
        },
        Err(e) => println!("   ✗ Failed to get default database: {}", e),
    }

    // Check if specific database exists
    println!("\n6. Checking database existence:");
    println!("   - Has 'main': {}", manager.has_database("main"));
    println!("   - Has 'replica': {}", manager.has_database("replica"));
    println!(
        "   - Has 'nonexistent': {}",
        manager.has_database("nonexistent")
    );

    // Gracefully close all connections
    println!("\n7. Closing all database connections...");
    let mut manager = manager;
    manager.close_all().await?;
    println!("   ✓ All connections closed");

    println!("\n=== Example completed ===");
    println!("Note: Some databases may fail to connect if they don't exist.");
    println!("This is expected behavior for demonstration purposes.");

    Ok(())
}
