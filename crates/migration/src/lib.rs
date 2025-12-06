pub use sea_orm_migration::prelude::*;

mod m01_prompt;
mod m02_users;
// mod s01_medical;
mod m03_blood_pressure_records;
mod m04_lipid_records;
mod m05_blood_routine_records;
mod m06_liver_function_records;
mod m07_renal_function_records;
mod m08_urine_records;
mod m09_body_records;
mod m10_sleep_records;
mod m11_other_metrics_records;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Box::new(s01_medical::Migration),
            Box::new(m01_prompt::Migration),
            Box::new(m02_users::Migration),
            Box::new(m03_blood_pressure_records::Migration),
            Box::new(m04_lipid_records::Migration),
            Box::new(m05_blood_routine_records::Migration),
            Box::new(m06_liver_function_records::Migration),
            Box::new(m07_renal_function_records::Migration),
            Box::new(m08_urine_records::Migration),
            Box::new(m09_body_records::Migration),
            Box::new(m10_sleep_records::Migration),
            Box::new(m11_other_metrics_records::Migration),
        ]
    }
}
