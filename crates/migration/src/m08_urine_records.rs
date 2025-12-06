use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UrineRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UrineRecords::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UrineRecords::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UrineRecords::Date).date().not_null())
                    // 尿常规指标，例如 GLU、PRO、KET、LEU 等
                    .col(ColumnDef::new(UrineRecords::Metrics).json_binary().null())
                    .col(text_null(UrineRecords::Source))
                    .col(
                        ColumnDef::new(UrineRecords::CreatedAt)
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
            .drop_table(Table::drop().table(UrineRecords::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UrineRecords {
    Table,
    Id,
    UserId,
    Date,
    Metrics,
    Source,
    CreatedAt,
}
