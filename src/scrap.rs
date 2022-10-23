mod keibagojp;
mod keibagojp_racecard;
mod write_racecard;

use crate::Racecourse;

use chrono::prelude::*;
use std::thread;
use thiserror::Error;

use self::keibagojp::scrap_keibagojp;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("failed to fetch from web site")]
    FetchingError,

    #[error("failed to fetch from web site")]
    NonBusinessDay,

    #[error("data store disconnected")]
    SelectorParseError,
}

#[derive(Debug)]
pub struct RaceData {
    race: i32,
    posttime: Option<String>,
    change: Option<String>,
    racetype: Option<String>,
    name: Option<String>,
    class: Option<String>,
    corse: Option<String>,
    weather: Option<String>,
    going: Option<String>,
    count: Option<String>,
}

//指定した日付・競馬場のデータをWebサイトから取得し、sqliteに書き込む
pub fn scrap(from_date: Date<Local>, to_date: Date<Local>, racecourse: Racecourse) {
    let mut date = from_date;
    loop {
        if to_date < date {
            break;
        }
        match scrap_keibagojp(date, &racecourse) {
            Ok(_) => (),
            Err(CustomError::NonBusinessDay) => (),
            Err(_) => {
                break;
            }
        };
        thread::sleep(std::time::Duration::from_secs(5));
        date = date + chrono::Duration::days(1);
    }
}
