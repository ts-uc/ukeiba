pub mod race;
use crate::enums::Racecourse;
use chrono::{Date, Local};

pub trait Db {
    fn upsert_sql(&self) -> String;
}

//関数の置き場所は暫定
fn gen_date(date: &Date<Local>) -> String {
    format!("{}", date.format("%Y-%m-%d"))
}

fn gen_raceid(date: &Date<Local>, racecourse: &Racecourse, race: &i32) -> i64 {
    format!(
        "{}{:02}{:02}",
        date.format("%Y%m%d"),
        racecourse.get_jravan_id(),
        race
    )
    .parse()
    .unwrap()
}

fn gen_racehorseid(
    date: &Date<Local>,
    racecourse: &Racecourse,
    race: &i32,
    horse_num: &i32,
) -> i64 {
    format!(
        "{}{:02}{:02}{:02}",
        date.format("%Y%m%d"),
        racecourse.get_jravan_id(),
        race,
        horse_num
    )
    .parse()
    .unwrap()
}
