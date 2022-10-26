//#![deny(warnings)]
mod common;
mod db;
mod enums;
mod fetch;
mod file;
mod scrap;

use crate::fetch::fetch_racelist;
use crate::scrap::keibagojp_racecard::scrap_racecard;
use chrono::prelude::*;
use db::insert_racecard;
use enums::*;
use file::*;
use std::thread;
//use scrap::scrap;
//use strum::IntoEnumIterator;

fn main() {
    env_logger::init();
    // let from_date = Local.ymd(2010, 4, 1);
    // let to_date = Local.ymd(2022, 10, 26);
    // let racecourse = Racecourse::Obihiro;

    // let mut date = from_date;
    // loop {
    //     if to_date < date {
    //         break;
    //     }
    //     match fetch_racelist(&date, &racecourse) {
    //         Ok(body) => {save_racelist(&date, &racecourse, &body);},
    //         Err(_) => {
    //             break;
    //         }
    //     };
    //     date = date + chrono::Duration::days(1);
    //     thread::sleep(std::time::Duration::from_secs(3));
    // }

    for item in racelist_dir().into_iter() {
        let path = item.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let date = Local.ymd(
            file_name[9..13].parse().unwrap(),
            file_name[13..15].parse().unwrap(),
            file_name[15..17].parse().unwrap(),
        );
        let racecourse = Racecourse::Obihiro;
        let body = std::fs::read_to_string(&path).unwrap();

        match scrap_racecard(&date, &racecourse, &body) {
            Ok(x) => {insert_racecard(&x);},
            Err(_) => (),
        }
    }

    // scrap(from_date, to_date, racecourse);

    // for racecourse in Racecourse::iter() {
    //     scrap(from_date, to_date, racecourse);
    // }
}
