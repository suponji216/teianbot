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
                    .table(Chat::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Chat::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Chat::Channel).integer().not_null())
                    .col(ColumnDef::new(Chat::Count).integer().not_null())
                    .col(ColumnDef::new(Chat::Mentioned).integer().not_null())
                    .col(ColumnDef::new(Chat::User).integer().not_null())
                    .col(
                        ColumnDef::new(Chat::Created)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Chat::Table, Chat::Channel)
                            .to(Channel::Table, Channel::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Chat::Table, Chat::User)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chat::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Chat {
    Table,
    Id,
    Channel,
    Count,
    Mentioned,
    User,
    Created,
}
