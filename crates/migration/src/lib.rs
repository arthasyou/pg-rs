pub use sea_orm_migration::prelude::*;

mod m0001_phase_a_core;
mod m0002_recipe;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m0001_phase_a_core::Migration),
            Box::new(m0002_recipe::Migration),
        ]
    }
}
