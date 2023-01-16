mod common;
mod db_reader;
mod db_writer;
mod reader;
mod webpage;
use crate::db_writer::DbType;
use crate::db_writer::Db;
use crate::common::race::Race;
use crate::reader::race::RaceReader;
use crate::reader::racelist::RaceListReader;
use crate::{common::date_racecourse::DateRacecourse, db_reader::get_racelist};
use chrono::{Duration, Local, NaiveDate};
use clap::{Parser, Subcommand};
use common::horse::Horse;
use db_reader::get_horselist;
use reader::oddspark_odds::OddsparkOddsReader;
use crate::common::racecourse::Racecourse;
use indicatif::ProgressBar;
use reader::horse_history::HorseHistoryReader;
use reader::horse_profile::HorseProfileReader;

/// Simple program to greet a person
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
    Race { racecouse: Option<Racecourse> },
    HorseHistory,
    HorseProfile,
    Odds,
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
            let pb = ProgressBar::new((day_count + 1).try_into().unwrap());
            let mut queries: Vec<DbType> = Vec::new();
            for day in 0..=day_count {
                pb.inc(1);
                let date = to_date - Duration::days(day);

                let dateracecourse = DateRacecourse::new(date, racecourse);
                queries.extend(RaceListReader::new(dateracecourse)
                .get(args.force_fetch, !args.not_save)
                .db()) ;
            }
            Db::new(queries).execute();
        }

        Mode::Race { racecouse: _ } => {
            let racelist = get_racelist(from_date, to_date);
            let pb = ProgressBar::new(racelist.len() as u64);
            let mut queries: Vec<DbType> = Vec::new();
            for race_id in racelist {
                pb.inc(1);
                let race = Race::from_race_id(race_id);
                queries.extend(RaceReader::new(race)
                    .get(args.force_fetch, !args.not_save)
                    .db());
            }
            Db::new(queries).execute();
        }

        Mode::HorseHistory => {
            let horselist = get_horselist(from_date, to_date);
            let pb = ProgressBar::new(horselist.len() as u64);
            let mut queries: Vec<DbType> = Vec::new();
            for horse_id in horselist {
                pb.inc(1);
                let horse = Horse::new(horse_id);
                queries.extend(HorseHistoryReader::new(horse).get(args.force_fetch, !args.not_save).db());
            }
            Db::new(queries).execute();
        }

        Mode::HorseProfile => {
            let horselist = get_horselist(from_date, to_date);
            let pb = ProgressBar::new(horselist.len() as u64);
            let mut queries: Vec<DbType> = Vec::new();
            for horse_id in horselist {
                pb.inc(1);
                let horse = Horse::new(horse_id);
                queries.extend(HorseProfileReader::new(horse).get(args.force_fetch, !args.not_save).db());
            }
            Db::new(queries).execute();
        }

        Mode::Odds => {
            let racelist = get_racelist(from_date, to_date);
            let pb = ProgressBar::new(racelist.len() as u64);
            let mut queries: Vec<DbType> = Vec::new();
            for race_id in racelist {
                pb.inc(1);
                let race = Race::from_race_id(race_id);
                queries.extend(OddsparkOddsReader::new(race)
                    .get(args.force_fetch, !args.not_save)
                    .db());
            }
            Db::new(queries).execute();
        }
    }
}
