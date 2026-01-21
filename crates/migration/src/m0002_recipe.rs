use sea_orm_migration::prelude::*;

use crate::m0001_phase_a_core::Metric;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Recipe::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Recipe::MetricId)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Recipe::Deps).json_binary().not_null())
                    .col(ColumnDef::new(Recipe::CalcKey).string().not_null())
                    .col(ColumnDef::new(Recipe::ArgMap).json_binary().not_null())
                    .col(ColumnDef::new(Recipe::Expr).json_binary().not_null())
                    .col(ColumnDef::new(Recipe::MetricCode).string().not_null())
                    .col(ColumnDef::new(Recipe::MetricName).string().not_null())
                    .col(ColumnDef::new(Recipe::Unit).string().null())
                    .col(ColumnDef::new(Recipe::ValueType).string().not_null())
                    .col(ColumnDef::new(Recipe::Visualization).string().not_null())
                    .col(ColumnDef::new(Recipe::Status).string().not_null())
                    .col(
                        ColumnDef::new(Recipe::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Recipe::Table, Recipe::MetricId)
                            .to(Metric::Table, Metric::MetricId),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Recipe::Table).cascade().to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Recipe {
    Table,
    MetricId,
    Kind,
    Deps,
    CalcKey,
    ArgMap,
    Expr,
    MetricCode,
    MetricName,
    Unit,
    ValueType,
    Visualization,
    Status,
    CreatedAt,
}
