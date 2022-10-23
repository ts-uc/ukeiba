//#![deny(warnings)]
mod scrap;
mod enums;

use chrono::prelude::*;
use scrap::scrap;
use enums::*;

fn main() {
    env_logger::init();
    let from_date = Local.ymd(2021, 4, 23);
    let to_date = Local.ymd(2022, 3, 31);
    let racecourse = Racecourse::Obihiro;

    scrap(from_date, to_date, racecourse);
}
