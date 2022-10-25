mod keibagojp;
mod keibagojp_racecard;
mod keibagojp_result;
mod write_racecard;

use crate::enums::*;

use chrono::prelude::*;

use self::{keibagojp::scrap_keibagojp, keibagojp_result::scrap_result};

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

#[derive(Debug)]
pub struct RaceResult {
    date: Date<Local>,
    racecourse: Racecourse,
    race: i32,
    horse_num: i32,
    bracket_num: Option<i32>,
    arrival: Option<i32>,
    horse_name: Option<String>,
    horse_id: Option<i64>,
    horse_affiliation: Option<String>,
    horse_sex: Option<String>,
    horse_age: Option<i32>,
    weight_to_carry: Option<i32>,
    jockey: Option<String>,
    jockey_id: Option<i32>,
    trainer: Option<String>,
    trainer_id: Option<i32>,
    horse_weight: Option<i32>,
    horse_weight_delta: Option<i32>,
    finish: Option<String>,
    margin: Option<String>,
    three_furlongs: Option<f32>,
    win_fav: Option<i32>,
}

//指定した日付・競馬場のデータをWebサイトから取得し、sqliteに書き込む
pub fn scrap(from_date: Date<Local>, to_date: Date<Local>, racecourse: Racecourse) {
    // scrap_result(&from_date, &racecourse, &7);
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
        date = date + chrono::Duration::days(1);
    }
}
