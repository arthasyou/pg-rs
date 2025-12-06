use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LipidRecords::Table)
                    .if_not_exists()
                    // 主键 id
                    .col(
                        ColumnDef::new(LipidRecords::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // 用户 id
                    .col(
                        ColumnDef::new(LipidRecords::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    // 记录日期
                    .col(ColumnDef::new(LipidRecords::MeasureTime).date().not_null())
                    // 甘油三酯
                    .col(ColumnDef::new(LipidRecords::TgMmol).decimal().null())
                    // 高密度脂蛋白
                    .col(ColumnDef::new(LipidRecords::HdlMmol).decimal().null())
                    // 低密度脂蛋白
                    .col(ColumnDef::new(LipidRecords::LdlMmol).decimal().null())
                    // 总胆固醇
                    .col(ColumnDef::new(LipidRecords::TcMmol).decimal().null())
                    // 额外信息统一 JSONB（扩展）
                    .col(
                        ColumnDef::new(LipidRecords::Extra)
                            .json_binary()
                            .default(Expr::value("{}"))
                            .not_null(),
                    )
                    // 数据来源
                    .col(text_null(LipidRecords::Source))
                    // 创建时间，默认当前时间
                    .col(
                        ColumnDef::new(LipidRecords::CreatedAt)
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
            .drop_table(Table::drop().table(LipidRecords::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum LipidRecords {
    Table,
    Id,
    UserId,
    MeasureTime,
    TgMmol,
    HdlMmol,
    LdlMmol,
    TcMmol,
    Extra,
    Source,
    CreatedAt,
}
