use crate::scrap::RaceData;
use crate::Racecourse;

use chrono::prelude::*;
use rusqlite::{Connection, Result};

fn gen_raceid(date: &Date<Local>, racecourse: &Racecourse, race: i32) -> i64 {
    format!(
        "{}{:02}{:02}",
        date.format("%Y%m%d"),
        match racecourse {
            Racecourse::Monbetsu => 30,
            Racecourse::Obihiro => 33,
            Racecourse::Morioka => 35,
            Racecourse::Mizusawa => 36,
            Racecourse::Kochi => 54,
            _ => todo!(),
        },
        race
    )
    .parse()
    .unwrap()
}

fn gen_date(date: &Date<Local>) -> String {
    format!("{}", date.format("%Y-%m-%d"))
}

fn gen_racecourse(racecourse: &Racecourse) -> String {
    match racecourse {
        Racecourse::Monbetsu => "門別",
        Racecourse::Obihiro => "帯広",
        Racecourse::Morioka => "盛岡",
        Racecourse::Mizusawa => "水沢",
        Racecourse::Kochi => "高知",
        _ => todo!(),
    }
    .to_string()
}

pub fn insert_racecard(date: &Date<Local>, racecourse: &Racecourse, racecard: &Vec<RaceData>) {
    let path = "./ukeiba.db3";
    let conn = Connection::open(&path).unwrap();

    for racedata in racecard {
        println!("{:?}", racedata);

        conn.execute(
            "INSERT INTO race (race_id, race_date, racecource, posttime, change, race_type, race_name, class, corse, weather, going, horse_count) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            (gen_raceid(date, racecourse, racedata.race), gen_date(date), gen_racecourse(racecourse), &racedata.posttime, &racedata.change, &racedata.racetype, &racedata.name, &racedata.class, &racedata.corse, &racedata.weather, &racedata.going, &racedata.count),
        ).unwrap();
    }
}
