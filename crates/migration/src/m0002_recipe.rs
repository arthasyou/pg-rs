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
                    .col(ColumnDef::new(Recipe::MetricCode).string().null())
                    .col(ColumnDef::new(Recipe::MetricName).string().null())
                    .col(ColumnDef::new(Recipe::Unit).string().null())
                    .col(ColumnDef::new(Recipe::ValueType).string().null())
                    .col(ColumnDef::new(Recipe::Visualization).string().null())
                    .col(ColumnDef::new(Recipe::Status).string().not_null())
                    .col(
                        ColumnDef::new(Recipe::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .check(Expr::cust(
                        "((kind = 'primitive' AND calc_key IS NULL AND arg_map IS NULL AND expr \
                         IS NULL AND metric_code IS NULL AND metric_name IS NULL AND unit IS NULL \
                         AND value_type IS NULL AND visualization IS NULL AND status IS \
                         NULL)OR(kind = 'derived' AND deps IS NOT NULL AND jsonb_typeof(deps) = \
                         'array' AND jsonb_array_length(deps) > 0 AND calc_key IS NOT NULL AND \
                         length(calc_key) > 0 AND metric_code IS NOT NULL AND length(metric_code) \
                         > 0 AND metric_name IS NOT NULL AND length(metric_name) > 0 AND \
                         value_type IS NOT NULL AND length(value_type) > 0 AND visualization IS \
                         NOT NULL AND length(visualization) > 0 AND status IS NOT NULL AND \
                         length(status) > 0))",
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_observation_recipe")
                    .from(Observation::Table, Observation::RecipeId)
                    .to(Recipe::Table, Recipe::RecipeId)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Cascade)
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

#[derive(DeriveIden)]
enum Observation {
    Table,
    RecipeId,
}
