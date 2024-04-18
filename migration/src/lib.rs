pub use sea_orm_migration::prelude::*;

mod m20240405_162109_create_table_users;
mod m20240405_162252_create_table_channels;
mod m20240405_162253_create_table_inout;
mod m20240405_170313_create_table_chat;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240405_162109_create_table_users::Migration),
            Box::new(m20240405_162252_create_table_channels::Migration),
            Box::new(m20240405_162253_create_table_inout::Migration),
            Box::new(m20240405_170313_create_table_chat::Migration),
        ]
    }
}
