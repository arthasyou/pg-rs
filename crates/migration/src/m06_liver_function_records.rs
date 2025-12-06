use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LiverFunctionRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LiverFunctionRecords::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(LiverFunctionRecords::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(LiverFunctionRecords::Date).date().not_null())
                    // 肝功能指标，可包括 ALT、AST、TBil、DBil、GGT 等
                    .col(
                        ColumnDef::new(LiverFunctionRecords::Metrics)
                            .json_binary()
                            .null(),
                    )
                    .col(text_null(LiverFunctionRecords::Source))
                    .col(
                        ColumnDef::new(LiverFunctionRecords::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LiverFunctionRecords::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum LiverFunctionRecords {
    Table,
    Id,
    UserId,
    Date,
    Metrics,
    Source,
    CreatedAt,
}
