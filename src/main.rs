mod common;
mod db_reader;
mod db_writer;
mod enums;
mod reader;
mod webpage;
use crate::common::race::Race;
use crate::reader::race::RaceReader;
use crate::{common::date_racecourse::DateRacecourse, db_reader::get_racelist};
use crate::db_writer::initialize::Initialize;
use crate::db_writer::Executer;
use chrono::{Duration, Local, NaiveDate};
use clap::{Parser, Subcommand};
use enums::Racecourse;
use indicatif::ProgressBar;

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
    Initialize::new().execute();

    match args.mode {
        Mode::Racelist { racecourse } => {
            let pb = ProgressBar::new((day_count + 1).try_into().unwrap());
            for day in 0..=day_count {
                pb.inc(1);
                let date = to_date - Duration::days(day);

                let dateracecourse = DateRacecourse {
                    date: date,
                    racecourse: racecourse,
                };

                log::info!("{:?}", dateracecourse);

                let racelist = dateracecourse
                    .make_racelist_reader()
                    .get(args.force_fetch, !args.not_save);
                racelist.db().execute();
            }
        }

        Mode::Race { racecouse } => {
            let racelist = get_racelist(from_date, to_date);
            let pb = ProgressBar::new(racelist.len() as u64);
            for race_id in racelist{
                pb.inc(1);
                let race = Race::from_race_id(race_id);
                RaceReader::new(race).get(args.force_fetch, !args.not_save).db().execute();
            }
        }
    }
}
