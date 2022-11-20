use crate::enums::*;
use crate::common::*;

use chrono::prelude::*;
use rusqlite::Connection;
use rusqlite::Statement;
use rusqlite::params;

fn gen_date(date: &Date<Local>) -> String {
    format!("{}", date.format("%Y-%m-%d"))
}

pub fn insert_racecard(racecard: &Vec<RaceData>) {
    let path = "./ukeiba.db3";
    let conn = Connection::open(&path).unwrap();

    for racedata in racecard {
        conn.execute(
            "REPLACE  INTO race (
                race_id, race_date, racecource, race_num, posttime,
                change, race_type, race_name, class, surface,
                direction, distance, weather, going, moisture,
                horse_count) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
            (
                gen_raceid(&racedata.date, &racedata.racecourse, &racedata.race),
                gen_date(&racedata.date),
                &racedata.racecourse.get_name(),
                &racedata.race,
                &racedata.posttime,
                //
                &racedata.change,
                &racedata.racetype,
                &racedata.name,
                &racedata.class,
                &racedata.surface,
                //
                &racedata.distance,
                &racedata.direction,
                &racedata.weather,
                &racedata.going,
                &racedata.moisture,
                //
                &racedata.count,
            ),
        )
        .unwrap();
    }
}

pub fn insert_result(racecard: &Vec<RaceResult>) {
    let path = "./ukeiba.db3";
    let conn = Connection::open(&path).unwrap();

    for racedata in racecard {
        conn.execute(
            "REPLACE INTO result (
                race_horse_id, race_id, horse_num, bracket_num, arrival,
                horse_name, horse_id, horse_affiliation, horse_sex, horse_age,
                weight_to_carry, jockey, jockey_id, trainer, trainer_id,
                horse_weight, horse_weight_delta, finish, margin, three_furlongs,
                win_fav) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 
                    ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21)",
            params![
                gen_racehorseid(&racedata.date, &racedata.racecourse, &racedata.race, &racedata.horse_num),
                gen_raceid(&racedata.date, &racedata.racecourse, &racedata.race),
                &racedata.horse_num,
                &racedata.bracket_num,
                &racedata.arrival,
                //
                &racedata.horse_name,
                &racedata.horse_id,
                &racedata.horse_affiliation,
                &racedata.horse_sex,
                &racedata.horse_age,
                //
                &racedata.weight_to_carry,
                &racedata.jockey,
                &racedata.jockey_id,
                &racedata.trainer,
                &racedata.trainer_id,
                //
                &racedata.horse_weight,
                &racedata.horse_weight_delta,
                &racedata.finish,
                &racedata.margin,
                &racedata.three_furlongs,
                //
                &racedata.win_fav,             
            ],
        )
        .unwrap();
    }
}

#[derive(Debug)]
pub struct RaceIdStruct{pub race_id: i64}

pub fn select_raceid () -> Vec<i64>{
    let path = "./ukeiba.db3";
    let conn = Connection::open(&path).unwrap();

    let mut stmt = conn.prepare("SELECT race_id FROM race").unwrap();
    let person_iter = stmt.query_map(params![], |row| {
        Ok(RaceIdStruct{race_id: row.get_unwrap(0)})
    }).unwrap();

    person_iter.map(|p| p.unwrap().race_id).collect()
    // for p in person_iter{
    //     println!("{:?}", p.unwrap().race_id);
    // }
}