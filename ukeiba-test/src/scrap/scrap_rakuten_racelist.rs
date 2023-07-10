use chrono::NaiveDate;
use ukeiba_common::scraper::rakuten_racelist;

use crate::{
    common::fetch_and_scrap_all,
    db::{
        writer::{write_to_db, DbWriter},
        Dates,
    },
    get::get_date_racecourse,
};

pub fn scrap() {
    let race_data =
        get_date_racecourse::get_all_from_db(NaiveDate::from_ymd_opt(2010, 01, 01).unwrap());

    let pages = race_data
        .into_iter()
        .map(|x| rakuten_racelist::Page {
            race_date: x.race_date,
            racecourse: x.racecourse,
        })
        .collect::<Vec<_>>();

    let data = fetch_and_scrap_all(pages);
    let mut db_writer: Vec<DbWriter> = Vec::new();

    for datum in data {
        db_writer.push(DbWriter::UpsertDates(Dates {
            race_date: datum.race_date,
            racecourse: Some(datum.racecourse.to_name()),
            kai: datum.kai,
            nichi: datum.nichi,
            ..Default::default()
        }));
    }
    write_to_db(&db_writer);
}
