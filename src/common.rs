#![allow(unused)]

use chrono::prelude::*;
use crate::enums::*;

#[derive(Debug)]
pub struct RaceData {
    pub date: Date<Local>,
    pub racecourse: Racecourse,
    pub race: i32,
    pub posttime: Option<String>,
    pub change: Option<String>,
    pub racetype: Option<String>,
    pub name: Option<String>,
    pub class: Option<String>,
    pub surface: Option<String>,
    pub direction: Option<String>,
    pub distance: Option<i32>,
    pub weather: Option<String>,
    pub going: Option<String>,
    pub moisture: Option<f64>,
    pub count: Option<i32>,
}

#[derive(Debug)]
pub struct RaceResult {
    pub date: Date<Local>,
    pub racecourse: Racecourse,
    pub race: i32,
    pub horse_num: i32,
    pub bracket_num: Option<i32>,
    pub arrival: Option<i32>,
    pub horse_name: Option<String>,
    pub horse_id: Option<i64>,
    pub horse_affiliation: Option<String>,
    pub horse_sex: Option<String>,
    pub horse_age: Option<i32>,
    pub weight_to_carry: Option<i32>,
    pub jockey: Option<String>,
    pub jockey_id: Option<i32>,
    pub trainer: Option<String>,
    pub  trainer_id: Option<i32>,
    pub horse_weight: Option<i32>,
    pub horse_weight_delta: Option<i32>,
    pub finish: Option<String>,
    pub margin: Option<String>,
    pub three_furlongs: Option<f32>,
    pub win_fav: Option<i32>,
}


pub fn gen_racelistid(date: &Date<Local>, racecourse: &Racecourse) -> i64 {
    format!(
        "{}{:02}",
        date.format("%Y%m%d"),
        racecourse.get_jravan_id(),
    )
    .parse()
    .unwrap()
}

pub fn gen_raceid(date: &Date<Local>, racecourse: &Racecourse, race: &i32) -> i64 {
    format!(
        "{}{:02}{:02}",
        date.format("%Y%m%d"),
        racecourse.get_jravan_id(),
        race
    )
    .parse()
    .unwrap()
}

pub fn gen_racehorseid(date: &Date<Local>, racecourse: &Racecourse, race: &i32, horse_num: &i32) -> i64 {
    format!(
        "{}{:02}{:02}{:02}",
        date.format("%Y%m%d"),
        racecourse.get_jravan_id(),
        race,
        horse_num
    )
    .parse()
    .unwrap()
}