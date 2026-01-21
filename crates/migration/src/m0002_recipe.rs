use sea_orm_migration::prelude::*;

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
                        ColumnDef::new(Recipe::RecipeId)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Recipe::Kind).string().not_null())
                    .col(ColumnDef::new(Recipe::Deps).json_binary().not_null())
                    .col(ColumnDef::new(Recipe::CalcKey).string().null())
                    .col(ColumnDef::new(Recipe::ArgMap).json_binary().null())
                    .col(ColumnDef::new(Recipe::Expr).json_binary().null())
                    .col(ColumnDef::new(Recipe::MetricCode).string().not_null())
                    .col(ColumnDef::new(Recipe::MetricName).string().not_null())
                    .col(ColumnDef::new(Recipe::Unit).string().not_null())
                    .col(ColumnDef::new(Recipe::ValueType).string().not_null())
                    .col(ColumnDef::new(Recipe::Visualization).string().not_null())
                    .col(ColumnDef::new(Recipe::Status).string().not_null())
                    .col(
                        ColumnDef::new(Recipe::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
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
    RecipeId,
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
