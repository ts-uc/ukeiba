use chrono::{NaiveDate, NaiveTime};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::time::Instant;
use ukeiba_common::scraper::horse_table;

use crate::{
    common::fetch_and_scrap_all,
    db::{
        writer::{write_to_db, DbWriter},
        RaceHorses, Races,
    },
    get::get_race_data,
};

pub fn scrap() {
    let race_data = get_race_data::get_all_from_db(NaiveDate::from_ymd_opt(1998, 01, 01).unwrap());
    let pages = race_data
        .into_iter()
        .map(|x| horse_table::Page {
            race_date: x.race_date,
            racecourse: x.racecourse,
            race_num: x.race_num,
        })
        .collect::<Vec<_>>();

    let data = fetch_and_scrap_all(pages);

    let start = Instant::now();

    let mut db_writer: Vec<DbWriter> = Vec::new();

    for datum in data {
        db_writer.push(DbWriter::HorseTableToRaces(Races {
            race_date: datum.race_date,
            race_num: datum.race_num,
            post_time: datum
                .post_time
                .and_then(|x| NaiveTime::parse_from_str(&x, "%H:%M").ok()),
            post_time_change: datum.post_time_change,
            race_sub_name: datum.race_sub_title,
            race_name: Some(datum.race_title),
            weather: datum.race_detail.weather,
            going: datum.race_detail.going.and_then(|x| x.parse().ok()),
            race_age: datum.race_detail.race_age,
            race_weight_type: datum.race_detail.race_weight_type,
            horse_count_entered: Some(datum.registered_horse_count),
            ..Default::default()
        }));
        for x in datum.data {
            db_writer.push(DbWriter::HorseTableToRaceHorses(RaceHorses {
                race_date: datum.race_date,
                race_num: datum.race_num,
                horse_num: x.horse_num,
                horse_nar_id: Some(x.horse_nar_id),
                bracket_num: x.bracket_num,
                horse_sex: x.horse_sex,
                jockey_nar_id: x.jockey_nar_id,
                weight_mark: x.horse_weight_mark,
                weight_to_carry: x.weight_to_carry,
                trainer_nar_id: x.trainer_nar_id,
                owner_name: x.owner_name,
                horse_weight: x.horse_weight,
                change: x.horse_change,
                ..Default::default()
            }));
        }
    }

    // let db_writer = data
    //     .par_iter()
    //     .map(|x| datum_to_db_writer(x))
    //     .collect::<Vec<_>>()
    //     .concat();

    let end = Instant::now();
    let duration = end - start;
    let seconds = duration.as_nanos();
    println!("Execution time: {} seconds", seconds);

    write_to_db(&db_writer);
}

fn datum_to_db_writer(datum: &horse_table::Data) -> Vec<DbWriter> {
    let mut db_writer: Vec<DbWriter> = Vec::new();
    db_writer.push(DbWriter::HorseTableToRaces(Races {
        race_date: datum.race_date,
        race_num: datum.race_num,
        post_time: datum
            .post_time
            .clone()
            .and_then(|x| NaiveTime::parse_from_str(&x, "%H:%M").ok()),
        post_time_change: datum.post_time_change,
        race_sub_name: datum.race_sub_title.clone(),
        race_name: Some(datum.race_title.clone()),
        weather: datum.race_detail.weather.clone(),
        going: datum.race_detail.going.clone().and_then(|x| x.parse().ok()),
        race_age: datum.race_detail.race_age.clone(),
        race_weight_type: datum.race_detail.race_weight_type.clone(),
        horse_count_entered: Some(datum.registered_horse_count),
        ..Default::default()
    }));
    for x in datum.data.clone() {
        db_writer.push(DbWriter::HorseTableToRaceHorses(RaceHorses {
            race_date: datum.race_date,
            race_num: datum.race_num,
            horse_num: x.horse_num,
            horse_nar_id: Some(x.horse_nar_id),
            bracket_num: x.bracket_num,
            horse_sex: x.horse_sex,
            jockey_nar_id: x.jockey_nar_id,
            weight_mark: x.horse_weight_mark,
            weight_to_carry: x.weight_to_carry,
            trainer_nar_id: x.trainer_nar_id,
            owner_name: x.owner_name,
            horse_weight: x.horse_weight,
            change: x.horse_change,
            ..Default::default()
        }));
    }
    db_writer
}
