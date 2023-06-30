use chrono::NaiveTime;
use ukeiba_common::scraper::horse_table;

use crate::{
    common::fetch_and_scrap_all,
    db::{
        writer::{write_to_db, DbWriter},
        Dates, RaceHorses, Races,
    },
    get::get_race_data,
};

pub fn scrap() {
    let race_data = get_race_data::get_after_1997_01_from_db();
    let pages = race_data
        .into_iter()
        .map(|x| horse_table::Page {
            race_date: x.race_date,
            racecourse: x.racecourse,
            race_num: x.race_num,
        })
        .collect::<Vec<_>>();

    let data = fetch_and_scrap_all(pages);
    let mut db_writer: Vec<DbWriter> = Vec::new();

    for datum in data {
        db_writer.push(DbWriter::UpsertDates(Dates {
            race_date: datum.race_date,
            racecourse: Some(datum.racecourse.to_name()),
            ..Default::default()
        }));
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
    write_to_db(&db_writer);
}
