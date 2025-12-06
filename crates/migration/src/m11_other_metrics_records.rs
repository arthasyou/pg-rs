use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OtherMetricsRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OtherMetricsRecords::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OtherMetricsRecords::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(OtherMetricsRecords::Date).date().not_null())
                    // 未知指标分类名称
                    .col(text_null(OtherMetricsRecords::Category))
                    // 动态指标数据
                    .col(
                        ColumnDef::new(OtherMetricsRecords::Metrics)
                            .json_binary()
                            .null(),
                    )
                    .col(text_null(OtherMetricsRecords::Source))
                    .col(
                        ColumnDef::new(OtherMetricsRecords::CreatedAt)
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
            .drop_table(Table::drop().table(OtherMetricsRecords::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum OtherMetricsRecords {
    Table,
    Id,
    UserId,
    Date,
    Category,
    Metrics,
    Source,
    CreatedAt,
}
