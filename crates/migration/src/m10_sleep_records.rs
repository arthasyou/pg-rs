use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SleepRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SleepRecords::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SleepRecords::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SleepRecords::Date).date().not_null())
                    // 睡眠时长（小时）
                    .col(ColumnDef::new(SleepRecords::DurationHours).decimal().null())
                    // 睡眠评分
                    .col(ColumnDef::new(SleepRecords::SleepScore).decimal().null())
                    .col(text_null(SleepRecords::Source))
                    .col(
                        ColumnDef::new(SleepRecords::CreatedAt)
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
            .drop_table(Table::drop().table(SleepRecords::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SleepRecords {
    Table,
    Id,
    UserId,
    Date,
    DurationHours,
    SleepScore,
    Source,
    CreatedAt,
}
