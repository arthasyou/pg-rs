use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BloodRoutineRecords::Table)
                    .if_not_exists()
                    // 主键 id
                    .col(
                        ColumnDef::new(BloodRoutineRecords::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // 用户 id
                    .col(
                        ColumnDef::new(BloodRoutineRecords::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    // 记录日期
                    .col(
                        ColumnDef::new(BloodRoutineRecords::MeasureTime)
                            .date()
                            .not_null(),
                    )
                    // 动态指标，如 {"WBC":6.2,"HGB":120,"PLT":210}
                    .col(
                        ColumnDef::new(BloodRoutineRecords::Metrics)
                            .json_binary()
                            .null(),
                    )
                    // 额外信息统一 JSONB（扩展）
                    .col(
                        ColumnDef::new(BloodRoutineRecords::Extra)
                            .json_binary()
                            .default(Expr::value("{}"))
                            .not_null(),
                    )
                    // 数据来源
                    .col(text_null(BloodRoutineRecords::Source))
                    // 创建时间，默认当前时间
                    .col(
                        ColumnDef::new(BloodRoutineRecords::CreatedAt)
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
            .drop_table(Table::drop().table(BloodRoutineRecords::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BloodRoutineRecords {
    Table,
    Id,
    UserId,
    MeasureTime,
    Metrics,
    Extra,
    Source,
    CreatedAt,
}
