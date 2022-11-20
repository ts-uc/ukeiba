use chrono::{Date, Local};
use crate::enums::Racecourse;
use crate::db::*;

use super::Db;

pub struct Race {
    pub race_date: Date<Local>,
    pub racecourse: Racecourse,
    pub race_num: i32,
    pub post_time: Option<String>,
    pub change: Option<String>,
    pub race_type: Option<String>,
    pub race_name: Option<String>,
    pub surface: Option<String>,
    pub direction: Option<String>,
    pub distance: Option<i32>,
    pub weather: Option<String>,
    pub going: Option<String>,
    pub moisture: Option<f64>,
    pub horse_count: Option<i32>,
}

