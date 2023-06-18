use db::{create_table, vacuum_database};

pub mod db;
pub mod scrap;

fn main() {
    create_table().unwrap();
    scrap::scrap_jockey::scrap();
    scrap::scrap_trainer::scrap();
    scrap::scrap_horse_profile::scrap();
    scrap::scrap_bajikyo_profile::scrap();
    scrap::scrap_bajikyo_pedigree::scrap();
    scrap::scrap_horse_history::scrap();
    vacuum_database().unwrap();
}
