mod keibagojp;
mod keibagojp_racecard;
mod write_racecard;

use crate::enums::*;

use chrono::prelude::*;
use std::thread;

use self::keibagojp::scrap_keibagojp;



#[derive(Debug)]
pub struct RaceData {
    date: Date<Local>,
    racecourse: Racecourse,
    race: i32,
    posttime: Option<String>,
    change: Option<String>,
    racetype: Option<String>,
    name: Option<String>,
    class: Option<String>,
    surface: Option<String>,
    direction: Option<String>,
    distance: Option<i32>,
    weather: Option<String>,
    going: Option<String>,
    moisture: Option<f64>,
    count: Option<i32>,
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
