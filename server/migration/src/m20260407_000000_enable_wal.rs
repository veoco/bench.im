use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 启用 WAL 模式
        manager
            .get_connection()
            .execute_unprepared("PRAGMA journal_mode = WAL")
            .await?;

        // 设置同步模式为 NORMAL（性能与安全的平衡）
        manager
            .get_connection()
            .execute_unprepared("PRAGMA synchronous = NORMAL")
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 回滚时恢复 DELETE 模式
        manager
            .get_connection()
            .execute_unprepared("PRAGMA journal_mode = DELETE")
            .await?;

        Ok(())
    }
}
