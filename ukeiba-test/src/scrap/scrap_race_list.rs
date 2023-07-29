use ukeiba_common::scraper::race_list;

use crate::{
    common::fetch_and_scrap_all,
    db::{
        writer::{write_to_db, DbWriter},
        Dates, RaceHorses, Races,
    },
    get::get_date_racecourse,
};

pub fn scrap() {
    let race_data = get_date_racecourse::get_all_available();

    let pages = race_data
        .into_iter()
        .map(|x| race_list::Page {
            race_date: x.race_date,
            racecourse: x.racecourse,
        })
        .collect::<Vec<_>>();

    let data = fetch_and_scrap_all(pages);
    let mut db_writer: Vec<DbWriter> = Vec::new();

    for datum in data {
        if datum.race_list.is_empty() {
            continue;
        }
        db_writer.push(DbWriter::UpsertDates(Dates {
            race_date: datum.race_date,
            racecourse: Some(datum.racecourse.to_name()),
            ..Default::default()
        }));

        for race in datum.race_list {
            db_writer.push(DbWriter::RaceListToRaces(Races {
                race_date: datum.race_date,
                race_num: race.race_num,
                race_type: match race.race_type.as_deref() {
                    Some("重賞") => Some(1),
                    Some("準重賞") => Some(2),
                    Some("特別") => Some(3),
                    Some("一般") => Some(4),
                    Some(_) | None => None,
                },
                ..Default::default()
            }))
        }

        for change in datum.change_list {
            db_writer.push(DbWriter::RaceListToRaceHorses(RaceHorses {
                race_date: datum.race_date,
                race_num: change.race_num,
                horse_num: change.horse_num,
                change: change.change,
                change_reason: change.change_reason,
                ..Default::default()
            }))
        }
    }
    write_to_db(&db_writer);
}
