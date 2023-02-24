//#![allow(unused)]
mod common;
mod db_reader;
mod db_writer;
mod webpage;
use crate::common::race::Race;
use crate::common::racecourse::Racecourse;
use crate::db_writer::Db;
use crate::db_writer::DbType;
use crate::{common::date_racecourse::DateRacecourse, db_reader::get_racelist};
use chrono::{Duration, Local, NaiveDate};
use clap::{Parser, Subcommand};
use common::horse::Horse;
use db_reader::get_horse_birthdate_parents_list;
use db_reader::get_horselist;
use indicatif::ProgressBar;
use webpage::bajikyo_search::BajikyoSearchPage;
use webpage::horse_history::HorseHistoryPage;
use webpage::horse_profile::HorseProfilePage;
use webpage::oddspark_odds::OddsparkOddsPage;
use webpage::race::RacePage;
use webpage::racelist::RacelistPage;
use webpage::rakuten_racelist::RakutenRacelistPage;
use webpage::WebPageTrait;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Debug
    #[arg(short, long)]
    debug: bool,

    /// Write to db
    #[arg(long)]
    db: bool,

    /// Fetching from web
    #[arg(long)]
    force_fetch: bool,

    #[arg(long)]
    not_save: bool,

    /// From date
    #[arg(long)]
    from: Option<String>,

    /// To date
    #[arg(long)]
    to: Option<String>,

    #[command(subcommand)]
    mode: Mode,
}

#[derive(Subcommand, Debug)]
enum Mode {
    Racelist { racecourse: Racecourse },
    RakutenRacelist { racecourse: Racecourse },
    Race { racecouse: Option<Racecourse> },
    HorseHistory,
    HorseProfile,
    Odds,
    BajikyoSearch,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    if args.debug {
        std::env::set_var("RUST_LOG", "debug");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }

    let from_date = match args.from {
        Some(ref value) => NaiveDate::parse_from_str(&value, "%Y-%m-%d").unwrap(),
        None => Local::today().naive_local() - Duration::days(1),
    };

    let to_date = match args.to {
        Some(ref value) => NaiveDate::parse_from_str(&value, "%Y-%m-%d").unwrap(),
        None => Local::today().naive_local() - Duration::days(1),
    };

    let day_count = (to_date - from_date).num_days();

    // validation
    if day_count < 0 {
        eprintln!("エラー: to は from よりも後の日付を指定してください。");
        panic!();
    }

    // hontai
    db_writer::initialize();

    match args.mode {
        Mode::Racelist { racecourse } => {
            let pagelist: Vec<RacelistPage> = (0..=day_count)
                .map(|x| to_date - Duration::days(x))
                .map(|race| DateRacecourse::new(race, racecourse))
                .map(|race| RacelistPage(race))
                .collect();
            routine(pagelist);
        }

        Mode::RakutenRacelist { racecourse } => {
            let pagelist: Vec<RakutenRacelistPage> = (0..=day_count)
                .map(|x| to_date - Duration::days(x))
                .map(|race| DateRacecourse::new(race, racecourse))
                .map(|race| RakutenRacelistPage(race))
                .collect();
            routine(pagelist);
        }

        Mode::Race { racecouse: _ } => {
            let pagelist: Vec<RacePage> = get_racelist(from_date, to_date)
                .iter()
                .map(|race| RacePage(*race))
                .collect();
            routine(pagelist);
        }

        Mode::HorseHistory => {
            let pagelist: Vec<HorseHistoryPage> = get_horselist(from_date, to_date)
                .iter()
                .map(|race| HorseHistoryPage(*race))
                .collect();
            routine(pagelist);
        }

        Mode::HorseProfile => {
            let pagelist: Vec<HorseProfilePage> = get_horselist(from_date, to_date)
                .iter()
                .map(|race| HorseProfilePage(*race))
                .collect();
            routine(pagelist);
        }

        Mode::BajikyoSearch => {
            let pagelist: Vec<BajikyoSearchPage> =
                get_horse_birthdate_parents_list(from_date, to_date)
                    .iter()
                    .map(|race| BajikyoSearchPage(race.clone()))
                    .collect();
            routine(pagelist);
        }

        Mode::Odds => {
            let pagelist: Vec<OddsparkOddsPage> = get_racelist(from_date, to_date)
                .iter()
                .map(|race| OddsparkOddsPage(*race))
                .collect();
            routine(pagelist);
        }
    }
}

fn routine<T>(pagelist: Vec<T>)
where
    T: WebPageTrait + Clone,
{
    let pb = ProgressBar::new(pagelist.len() as u64);
    for race in pagelist.clone() {
        pb.inc(1);
        race.check_and_fetch();
    }
    let pb = ProgressBar::new(pagelist.len() as u64);
    let mut queries: Vec<DbType> = Vec::new();
    for race in pagelist.clone() {
        pb.inc(1);
        queries.extend(race.load().db())
    }
    Db::new(queries).execute();
}
