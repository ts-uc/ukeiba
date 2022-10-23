use crate::scrap::RaceData;
use crate::enums::*;

use chrono::prelude::*;
use rusqlite::{Connection};

fn gen_raceid(date: Date<Local>, racecourse: Racecourse, race: i32) -> i64 {
    format!(
        "{}{:02}{:02}",
        date.format("%Y%m%d"),
        racecourse.get_jravan_id(),
        race
    )
    .parse()
    .unwrap()
}

fn gen_date(date: &Date<Local>) -> String {
    format!("{}", date.format("%Y-%m-%d"))
}

pub fn insert_racecard(date: &Date<Local>, racecourse: &Racecourse, racecard: &Vec<RaceData>) {
    let path = "./ukeiba.db3";
    let conn = Connection::open(&path).unwrap();

    for racedata in racecard {
        conn.execute(
            "INSERT INTO race (race_id, race_date, racecource, race_num, posttime, change, race_type, race_name, class, surface, direction, distance, weather, going, moisture , horse_count) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
            (gen_raceid(racedata.date, racedata.racecourse, racedata.race), gen_date(&racedata.date), racecourse.get_name(), &racedata.race, &racedata.posttime, &racedata.change, &racedata.racetype, &racedata.name, &racedata.class, &racedata.surface, &racedata.distance, &racedata.direction, &racedata.weather, &racedata.going, &racedata.moisture, &racedata.count),
        ).unwrap();
    }
}
