use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Create subject table
        manager
            .create_table(
                Table::create()
                    .table(Subject::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Subject::SubjectId)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Subject::SubjectType)
                            .string()
                            .not_null()
                            .comment("Type: user, member, or future extension"),
                    )
                    .col(
                        ColumnDef::new(Subject::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 2. Create metric table
        manager
            .create_table(
                Table::create()
                    .table(Metric::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Metric::MetricId)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Metric::MetricCode)
                            .string()
                            .not_null()
                            .unique_key()
                            .comment("Stable identifier like 'blood_pressure_systolic'"),
                    )
                    .col(
                        ColumnDef::new(Metric::MetricName)
                            .string()
                            .not_null()
                            .comment("Display name"),
                    )
                    .col(
                        ColumnDef::new(Metric::Unit)
                            .string()
                            .null()
                            .comment("Unit of measurement, e.g., 'mmHg', 'mg/dL'"),
                    )
                    .col(
                        ColumnDef::new(Metric::ValueType)
                            .string()
                            .not_null()
                            .comment("Type: int, float, string"),
                    )
                    .col(
                        ColumnDef::new(Metric::Visualization)
                            .string()
                            .not_null()
                            .comment(
                                "Visualization type: line_chart, bar_chart, value_list, \
                                 single_value",
                            ),
                    )
                    .col(
                        ColumnDef::new(Metric::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 3. Create data_source table
        manager
            .create_table(
                Table::create()
                    .table(DataSource::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DataSource::SourceId)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DataSource::SourceType)
                            .string()
                            .not_null()
                            .comment("Type: manual, device, report"),
                    )
                    .col(
                        ColumnDef::new(DataSource::SourceName)
                            .string()
                            .not_null()
                            .comment("Name or description of the data source"),
                    )
                    .col(
                        ColumnDef::new(DataSource::Metadata)
                            .json_binary()
                            .null()
                            .comment("Optional JSON metadata"),
                    )
                    .col(
                        ColumnDef::new(DataSource::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 4. Create observation table (core fact table)
        manager
            .create_table(
                Table::create()
                    .table(Observation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Observation::ObservationId)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Observation::SubjectId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Observation::RecipeId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Observation::Value)
                            .string()
                            .not_null()
                            .comment(
                                "Value stored as string, parsed according to recipe.value_type",
                            ),
                    )
                    .col(
                        ColumnDef::new(Observation::ObservedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .comment("When the fact occurred"),
                    )
                    .col(
                        ColumnDef::new(Observation::RecordedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("When the fact was recorded in the system"),
                    )
                    .col(
                        ColumnDef::new(Observation::SourceId)
                            .big_integer()
                            .null()
                            .comment("Optional reference to data source"),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes for better query performance
        manager
            .create_index(
                Index::create()
                    .name("idx_observation_subject_recipe")
                    .table(Observation::Table)
                    .col(Observation::SubjectId)
                    .col(Observation::RecipeId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_observation_observed_at")
                    .table(Observation::Table)
                    .col(Observation::ObservedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order (respecting foreign keys)
        manager
            .drop_table(Table::drop().table(Observation::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(DataSource::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Metric::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Subject::Table).to_owned())
            .await?;

        Ok(())
    }
}

// Table identifier enums
#[derive(DeriveIden)]
enum Subject {
    Table,
    SubjectId,
    SubjectType,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Metric {
    Table,
    MetricId,
    MetricCode,
    MetricName,
    Unit,
    ValueType,
    Visualization,
    CreatedAt,
}

#[derive(DeriveIden)]
enum DataSource {
    Table,
    SourceId,
    SourceType,
    SourceName,
    Metadata,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Observation {
    Table,
    ObservationId,
    SubjectId,
    RecipeId,
    Value,
    ObservedAt,
    RecordedAt,
    SourceId,
}
