#![allow(unused)]

use chrono::prelude::*;
use crate::enums::*;

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