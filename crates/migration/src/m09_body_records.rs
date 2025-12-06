use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BodyRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BodyRecords::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BodyRecords::UserId).big_integer().not_null())
                    .col(ColumnDef::new(BodyRecords::Date).date().not_null())
                    // 体重（kg）
                    .col(ColumnDef::new(BodyRecords::WeightKg).decimal().null())
                    // 身高（cm）
                    .col(ColumnDef::new(BodyRecords::HeightCm).decimal().null())
                    // BMI
                    .col(ColumnDef::new(BodyRecords::Bmi).decimal().null())
                    .col(text_null(BodyRecords::Source))
                    .col(
                        ColumnDef::new(BodyRecords::CreatedAt)
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
            .drop_table(Table::drop().table(BodyRecords::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BodyRecords {
    Table,
    Id,
    UserId,
    Date,
    WeightKg,
    HeightCm,
    Bmi,
    Source,
    CreatedAt,
}
