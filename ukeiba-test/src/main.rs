extern crate ukeiba_common;
use db::{create_table, update_race_align, vacuum_database};
pub mod common;
pub mod db;
pub mod get;
use clap::{Parser, Subcommand};
pub mod scrap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    mode: Mode,
}

#[derive(Subcommand, Debug)]
enum Mode {
    Init,
    Tmp,
}

fn main() {
    let args = Args::parse();

    create_table().unwrap();
    match args.mode {
        Mode::Init => {
            scrap::scrap_trainers::scrap();
            scrap::scrap_jockeys::scrap();
            scrap::scrap_horse_profile::scrap();
            scrap::scrap_bajikyo_profile::scrap();
            scrap::scrap_bajikyo_pedigree::scrap();
            scrap::scrap_horse_history::scrap();
            scrap::scrap_horse_table::scrap();
            scrap::scrap_rakuten_racelist::scrap();
        }
        Mode::Tmp => {
            scrap::scrap_horse_history::scrap_active();
            scrap::scrap_horse_table::scrap();
            scrap::scrap_rakuten_racelist::scrap();
        }
    }
    update_race_align().unwrap();
    vacuum_database().unwrap();
}
