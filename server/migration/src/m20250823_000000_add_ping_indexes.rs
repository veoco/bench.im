use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("idx_ping_machine_target")
                    .table(Ping::Table)
                    .col(Ping::MachineId)
                    .col(Ping::TargetId)
                    .col(Ping::Ipv6)
                    .col(Ping::Created)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ping_target")
                    .table(Ping::Table)
                    .col(Ping::TargetId)
                    .col(Ping::Created)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ping_created")
                    .table(Ping::Table)
                    .col(Ping::Created)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_ping_machine_target").table(Ping::Table).to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_ping_target").table(Ping::Table).to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_ping_created").table(Ping::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Ping {
    Table,
    MachineId,
    TargetId,
    Ipv6,
    Created,
}
