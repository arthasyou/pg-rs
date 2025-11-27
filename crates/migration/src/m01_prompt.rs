use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Prompt::Table)
                    .if_not_exists()
                    // 主键 id
                    .col(
                        ColumnDef::new(Prompt::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // 标题
                    .col(string(Prompt::Title))
                    // prompt 内容
                    .col(text(Prompt::Content))
                    // 当前版本号（例如 1,2,3...）
                    .col(integer(Prompt::Version))
                    // 原始 prompt 的根 id
                    // 第一版时等于 id，之后版本指向这一条
                    .col(big_integer_null(Prompt::ParentId))
                    // 是否是最新版本
                    .col(boolean(Prompt::IsActive))
                    // 标签（可空）
                    .col(string_null(Prompt::Tags))
                    // 时间字段
                    .col(timestamp_with_time_zone(Prompt::CreateTime))
                    .col(timestamp_with_time_zone(Prompt::UpdateTime))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Prompt::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Prompt {
    Table,
    Id,
    Title,
    Content,
    Version,
    ParentId,
    IsActive,
    Tags,
    CreateTime,
    UpdateTime,
}
