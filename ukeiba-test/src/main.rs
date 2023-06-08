pub mod db;
pub mod scrap;

fn main() {
    db::Db::new().unwrap().create_table().unwrap();
    scrap::scrap_horse_profile::scrap_horse_profile();
    db::Db::new().unwrap().vacuum_database().unwrap();
}
