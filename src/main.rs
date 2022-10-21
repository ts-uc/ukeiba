//#![deny(warnings)]
mod scrap_rakuten_banei;

use chrono::prelude::*;

#[derive(Debug)]
pub enum Surface {
    Turf,
    Dirt,
}

#[derive(Debug)]
pub enum Going {
    GoodToFirm,
    Good,
    Yielding,
    Soft,
}

#[derive(Debug)]
pub struct RaceData {
    date: Date<Local>,
    racecourse: Racecourse,
    race: i32,
    th: Option<i32>,
    day: Option<i32>,
    surface: Option<Surface>,
    distance: Option<i32>,
    weather: Option<String>,
    going: Option<Going>,
    moisture: Option<f64>,
    posttime: Option<String>,
    name: Option<String>,
    class: Option<String>,
    breed: Option<String>,
    age: Option<String>,
}

#[derive(Debug)]
pub enum Racecourse {
    Obihiro,
    Monbetsu,
}

pub struct Race {
    date: Date<Local>,
    racecourse: Racecourse,
    num: i32,
}

fn main() {
    env_logger::init();
    let puri_puri_pudding = Race {
        date: Local.ymd(2022, 8, 20),
        racecourse: Racecourse::Obihiro,
        num: 7,
    };

    scrap_rakuten_banei::scrap(puri_puri_pudding);
}

