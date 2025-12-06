use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BloodPressureRecords::Table)
                    .if_not_exists()
                    // 主键 id
                    .col(
                        ColumnDef::new(BloodPressureRecords::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // 用户 id
                    .col(
                        ColumnDef::new(BloodPressureRecords::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    // 记录日期
                    .col(
                        ColumnDef::new(BloodPressureRecords::MeasureTime)
                            .date()
                            .not_null(),
                    )
                    // 核心血压数值（固定）
                    // 收缩压
                    .col(
                        ColumnDef::new(BloodPressureRecords::Systolic)
                            .integer()
                            .not_null(),
                    )
                    // 舒张压
                    .col(
                        ColumnDef::new(BloodPressureRecords::Diastolic)
                            .integer()
                            .not_null(),
                    )
                    // 额外信息统一 JSONB（扩展）
                    .col(
                        ColumnDef::new(BloodPressureRecords::Extra)
                            .json_binary()
                            .default(Expr::value("{}"))
                            .not_null(),
                    )
                    // 数据来源（md/json/manual）
                    .col(text_null(BloodPressureRecords::Source))
                    // 创建时间，默认当前时间
                    .col(
                        ColumnDef::new(BloodPressureRecords::CreatedAt)
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
            .drop_table(
                Table::drop()
                    .table(BloodPressureRecords::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum BloodPressureRecords {
    Table,
    Id,
    UserId,
    MeasureTime,
    Systolic,
    Diastolic,
    Extra,
    Source,
    CreatedAt,
}
