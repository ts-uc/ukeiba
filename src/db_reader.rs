use crate::{common::horse_birthdate_parents::HorseBirthdateParents, Horse, Race};
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

pub fn get_horse_birthdate_parents_list(from: NaiveDate, to: NaiveDate) -> Vec<HorseBirthdateParents> {
    let conn = get_conn();
    let sql = format!(
        "select distinct horse_nar_id, horses.horse_name, horse_birthdate, sire_name, dam_name from date_racecourses
        inner join races 
        on date_racecourses.date_racecourse_id = races.date_racecourse_id
        inner join race_horses on races.race_id = race_horses.race_id
        inner join horses on race_horses.horse_id = horses.horse_nar_id
        where '{}' <= race_date and race_date <= '{}'",
        from.to_string(),
        to.to_string()
    );
    let mut stmt = conn.prepare(&sql).unwrap();
    let data = stmt
        .query_map([], |row| {
            Ok(HorseBirthdateParents {
                horse: Horse::new(row.get(0).unwrap()),
                horse_name: row.get(1).unwrap(),
                birthdate: row.get(2).unwrap(),
                sire_name: row.get(3).unwrap(),
                dam_name: row.get(4).unwrap(),
            })
        })
        .unwrap()
        .map(|d| d.unwrap())
        .collect();
    data
}
