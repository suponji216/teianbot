use crate::{
    m20240405_162109_create_table_users::User, m20240405_162252_create_table_channels::Channel,
};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Inout::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Inout::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Inout::User).integer().not_null())
                    .col(ColumnDef::new(Inout::Channel).integer().not_null())
                    .col(
                        ColumnDef::new(Inout::In)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Inout::Out).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Inout::Table, Inout::User)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Inout::Table, Inout::Channel)
                            .to(Channel::Table, Channel::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Inout::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Inout {
    Table,
    Id,
    User,
    Channel,
    In,
    Out,
}
