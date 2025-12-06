use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RenalFunctionRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RenalFunctionRecords::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RenalFunctionRecords::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(RenalFunctionRecords::Date).date().not_null())
                    // 肾功能指标，Cr、BUN、UA、eGFR 等
                    .col(
                        ColumnDef::new(RenalFunctionRecords::Metrics)
                            .json_binary()
                            .null(),
                    )
                    .col(text_null(RenalFunctionRecords::Source))
                    .col(
                        ColumnDef::new(RenalFunctionRecords::CreatedAt)
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
            .drop_table(Table::drop().table(RenalFunctionRecords::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RenalFunctionRecords {
    Table,
    Id,
    UserId,
    Date,
    Metrics,
    Source,
    CreatedAt,
}
