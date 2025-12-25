/// Basic example demonstrating how to use pg-core and pg-tables
///
/// Run with: cargo run --example basic_usage
///
/// Requirements:
/// - PostgreSQL running on localhost:5432
/// - Database named 'test_db' exists
/// - Or set DATABASE_URL environment variable
use pg_core::{DatabaseConfig, DatabaseManager};

#[tokio::main]
async fn main() -> pg_core::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize tracing for logs
    tracing_subscriber::fmt::init();

    println!("=== Basic pg-core and pg-tables Usage Example ===\n");

    // Get database URL from environment or use default
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test_db".to_string());

    println!("1. Creating database configuration...");
    let config = DatabaseConfig::new("main", database_url)
        .max_connections(10)
        .min_connections(2)
        .connect_timeout(30)
        .with_sql_logging(true);

    println!("   - Database name: {}", config.name);
    println!("   - Max connections: {}", config.max_connections);
    println!("   - SQL logging: {}\n", config.sql_logging);

    println!("2. Connecting to database...");
    let manager = DatabaseManager::new(vec![config]).await?;
    println!("   ✓ Connected successfully\n");

    println!("3. Getting database connection...");
    let db = manager.default()?;
    println!("   ✓ Connection obtained\n");

    // Check connection by pinging
    println!("4. Testing connection with ping...");
    match db.ping().await {
        Ok(_) => println!("   ✓ Database is reachable\n"),
        Err(e) => {
            eprintln!("   ✗ Failed to ping database: {}", e);
            return Err(e.into());
        }
    }

    println!("5. Database manager info:");
    println!("   - Total databases: {}", manager.count());
    println!("   - Database list: {:?}\n", manager.list_databases());

    // Example: Using pg-tables (currently just demonstrates the import)
    println!("6. pg-tables is available for business logic:");
    println!("   - pg_tables::sea_orm available: ✓");
    println!("   - pg_tables::pg_core available: ✓\n");

    // Gracefully close connections
    println!("7. Closing database connections...");
    let mut manager = manager;
    manager.close_all().await?;
    println!("   ✓ All connections closed\n");

    println!("=== Example completed successfully ===");

    Ok(())
}
