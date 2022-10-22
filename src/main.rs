//#![deny(warnings)]
mod scrap;

use chrono::prelude::*;
use scrap::scrap;

#[derive(Debug)]
pub enum Racecourse {
    Obihiro,
    Monbetsu,
    Morioka,
    Mizusawa,
    Urawa,
    Funabashi,
    Ohi,
    Kawasaki,
    Kanazawa,
    Kasamatsu,
    Nagoya,
    Sonoda,
    Himeji,
    Kochi,
    Saga,
}

fn main() {
    env_logger::init();
    let from_date = Local.ymd(2021, 10, 21);
    let to_date = Local.ymd(2022, 10, 21);
    let racecourse = Racecourse::Obihiro;

    scrap(from_date, to_date, racecourse);
}
