//#![deny(warnings)]
mod scrap_rakuten_banei;

use chrono::prelude::*;

#[derive(Debug)]
pub enum Racecourse {
    Obihiro,
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

    let _ = scrap_rakuten_banei::scrap(puri_puri_pudding);
}

