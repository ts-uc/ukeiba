//#![deny(warnings)]
mod enums;
mod scrap;

use chrono::prelude::*;
use enums::*;
use scrap::scrap;
use strum::IntoEnumIterator;

fn main() {
    env_logger::init();
    let from_date = Local.ymd(2021, 1, 1);
    let to_date = Local.ymd(2021, 12, 31);
    let racecourse = Racecourse::Obihiro;
    scrap(from_date, to_date, racecourse);

    // for racecourse in Racecourse::iter() {
    //     scrap(from_date, to_date, racecourse);
    // }

}
