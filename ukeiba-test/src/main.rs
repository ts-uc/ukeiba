pub mod db;
pub mod scrap;

fn main() {
    db::Db::new().unwrap().create_table().unwrap();
    scrap::scrap_horse_profile::scrap();
    scrap::scrap_bajikyo_profile::scrap();
    scrap::scrap_bajikyo_pedigree::scrap();
    scrap::scrap_horse_history::scrap();
    db::Db::new().unwrap().vacuum_database().unwrap();
}
