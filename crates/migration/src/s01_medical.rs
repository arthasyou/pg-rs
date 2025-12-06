use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 使用原始 SQL 创建 medical schema
        manager
            .get_connection()
            .execute_unprepared("CREATE SCHEMA IF NOT EXISTS medical")
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 使用原始 SQL 删除 medical schema
        manager
            .get_connection()
            .execute_unprepared("DROP SCHEMA IF EXISTS medical CASCADE")
            .await?;
        Ok(())
    }
}
