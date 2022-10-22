//#![deny(warnings)]
mod scrap;

use chrono::prelude::*;

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
    let date = Local.ymd(2022, 8, 20);
    let racecourse = Racecourse::Obihiro;
}
