use crate::{Horse, Race};
use chrono::NaiveDate;
use rusqlite::Connection;

fn get_conn() -> Connection {
    let db_path = dirs::data_dir().unwrap().join("ukeiba").join("ukeiba.db");
    Connection::open(&db_path).unwrap()
}

pub fn get_racelist(from: NaiveDate, to: NaiveDate) -> Vec<Race> {
    let conn = get_conn();
    let sql = format!(
        "SELECT race_id FROM date_racecourses
        inner join races 
        on date_racecourses.date_racecourse_id = races.date_racecourse_id
        where '{}' <= race_date and race_date <= '{}'",
        from.to_string(),
        to.to_string()
    );
    let mut stmt = conn.prepare(&sql).unwrap();
    let data = stmt
        .query_map([], |row| Ok(Race::from_race_id(row.get(0).unwrap())))
        .unwrap()
        .map(|d| d.unwrap())
        .collect();
    data
}

pub fn get_horselist(from: NaiveDate, to: NaiveDate) -> Vec<Horse> {
    let conn = get_conn();
    let sql = format!(
        "select distinct horse_id from date_racecourses
        inner join races 
        on date_racecourses.date_racecourse_id = races.date_racecourse_id
        inner join race_horses on races.race_id = race_horses.race_id
        where '{}' <= race_date and race_date <= '{}'",
        from.to_string(),
        to.to_string()
    );
    let mut stmt = conn.prepare(&sql).unwrap();
    let data = stmt
        .query_map([], |row| Ok(Horse::new(row.get(0).unwrap())))
        .unwrap()
        .map(|d| d.unwrap())
        .collect();
    data
}
