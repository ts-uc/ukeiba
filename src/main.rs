mod common;
mod enums;
mod reader;
//mod webpage;
use chrono::{Duration, Local, NaiveDate, TimeZone};
use clap::{Parser, ValueEnum};
use enums::Racecourse;

use crate::{common::{date_racecourse::DateRacecourse}, reader::Reader};

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

    /// From date
    #[arg(long)]
    from: Option<String>,

    /// To date
    #[arg(long)]
    to: Option<String>,

    /// Racecourse Name
    #[arg(long, value_enum)]
    racecourse: Option<Racecourse>,

    #[arg(value_enum)]
    mode: Mode,
}

#[derive(ValueEnum, Debug, Clone, PartialEq)]
enum Mode {
    Racelist,
    Race,
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let args = Args::parse();

    let from_date = match args.from {
        Some(ref value) => {
            let native_date = NaiveDate::parse_from_str(&value, "%Y-%m-%d").unwrap();
            Local.from_local_date(&native_date).unwrap()
        }
        None => Local::today() - Duration::days(1),
    };

    let to_date = match args.to {
        Some(ref value) => {
            let native_date = NaiveDate::parse_from_str(&value, "%Y-%m-%d").unwrap();
            Local.from_local_date(&native_date).unwrap()
        }
        None => Local::today() - Duration::days(1),
    };

    // validation
    if to_date < from_date {
        eprintln!("エラー: to は from よりも後の日付を指定してください。");
        panic!();
    }

    if args.mode == Mode::Racelist && args.racecourse == None{
        eprintln!("modeがRacelist のときは、Racecourseが必須項目となります");
        panic!();
    }

    // hontai
    if args.mode == Mode::Racelist {
        let mut date = to_date;
        let racecourse = args.racecourse.unwrap();
        loop {
            if date < from_date {
                break;
            }

            let dateracecourse = DateRacecourse {
                date: date,
                racecourse: racecourse,
            };

            log::info!("{:?}", dateracecourse);

            let racelist = dateracecourse.make_racelist_reader();
            racelist.get_save_string(args.force_fetch);
        
            date = date - chrono::Duration::days(1);
        }
    }

}
