// 为 transfer_item 增加文件引用释放标记。
// 该字段用于恢复对齐时提前释放消失/变更子项的 file_cache 引用，并避免最终完成时重复扣减。

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table("transfer_item")
                    .add_column(
                        ColumnDef::new("file_ref_released")
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table("transfer_item")
                    .drop_column("file_ref_released")
                    .to_owned(),
            )
            .await
    }
}
