extern crate ukeiba_common;
use db::{create_table, update_race_align, vacuum_database};
pub mod common;
pub mod db;
pub mod get;
pub mod scrap;

fn main() {
    create_table().unwrap();
    scrap::scrap_trainers::scrap();
    scrap::scrap_jockeys::scrap();
    scrap::scrap_horse_profile::scrap();
    scrap::scrap_bajikyo_profile::scrap();
    scrap::scrap_bajikyo_pedigree::scrap();
    scrap::scrap_horse_history::scrap();
    scrap::scrap_horse_table::scrap();
    scrap::scrap_rakuten_racelist::scrap();
    update_race_align().unwrap();
    vacuum_database().unwrap();
}
