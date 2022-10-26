//#![deny(warnings)]
mod enums;
mod fetch;
mod common;
mod save;
// mod scrap;

use chrono::prelude::*;
use enums::*;
use save::save_racelist;
use std::thread;
use crate::fetch::fetch_racelist;
//use scrap::scrap;
//use strum::IntoEnumIterator;

fn main() {
    env_logger::init();
    let from_date = Local.ymd(2010, 4, 1);
    let to_date = Local.ymd(2022, 10, 26);
    let racecourse = Racecourse::Obihiro;

    let mut date = from_date;
    loop {
        if to_date < date {
            break;
        }
        match fetch_racelist(&date, &racecourse) {
            Ok(body) => {save_racelist(&date, &racecourse, &body);},
            Err(_) => {
                break;
            }
        };
        date = date + chrono::Duration::days(1);
        thread::sleep(std::time::Duration::from_secs(3));
    }

    // scrap(from_date, to_date, racecourse);

    // for racecourse in Racecourse::iter() {
    //     scrap(from_date, to_date, racecourse);
    // }

}
