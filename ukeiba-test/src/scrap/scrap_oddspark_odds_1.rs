use crate::{
    common::fetch_and_scrap_all,
    db::{
        writer::{write_to_db, DbWriter},
        RaceHorses,
    },
    get::get_race_data,
};
use chrono::NaiveDate;
use ukeiba_common::scraper::oddspark_odds_1;

pub fn scrap() {
    let race_data = get_race_data::get_all_from_db(NaiveDate::from_ymd_opt(2007, 04, 01).unwrap());
    let pages = race_data
        .into_iter()
        .map(|x| oddspark_odds_1::Page {
            race_date: x.race_date,
            racecourse: x.racecourse,
            race_num: x.race_num,
        })
        .collect::<Vec<_>>();

    let data = fetch_and_scrap_all(pages);
    let mut db_writer: Vec<DbWriter> = Vec::new();

    for datum in data {
        for datum2 in datum.odds_win_place_show {
            db_writer.push(DbWriter::OddsParkToRaceHorses(RaceHorses {
                race_date: datum.race_date,
                race_num: datum.race_num,
                horse_num: datum2.horse_num,
                win_odds: datum2.odds_win,
                place_odds_min: datum2.odds_place_show_min,
                place_odds_max: datum2.odds_place_show_max,
                ..Default::default()
            }));
        }
    }
    write_to_db(&db_writer);
}
