//#![deny(warnings)]
mod enums;
mod scrap;

use chrono::prelude::*;
use enums::*;
use scrap::scrap;
//use strum::IntoEnumIterator;

fn main() {
    env_logger::init();
    let from_date = Local.ymd(2022, 8, 20);
    let to_date = Local.ymd(2022, 10, 20);
    let racecourse = Racecourse::Obihiro;
    scrap(from_date, to_date, racecourse);

    // for racecourse in Racecourse::iter() {
    //     scrap(from_date, to_date, racecourse);
    // }

}
