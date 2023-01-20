#![allow(dead_code)]
use super::race::Race;
use super::*;
use crate::common::racecourse::Racecourse;
use chrono::{Datelike, NaiveDate};
use std::{fmt, path::PathBuf};

#[derive(Debug, Clone, Copy)]
pub struct DateRacecourse {
    pub date: NaiveDate,
    pub racecourse: Racecourse,
}

impl fmt::Display for DateRacecourse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_date_racecourse_id())
    }
}

impl DateRacecourse {
    pub fn new(date: NaiveDate, racecourse: Racecourse) -> Self {
        Self {
            date: date,
            racecourse: racecourse,
        }
    }

    pub fn from_date_racecourse_id(race_id: i64) -> Self {
        let year = race_id / 1000000;
        let month = (race_id / 10000) % 100;
        let day = (race_id / 100) % 100;
        let racecourse = race_id % 100;

        let year = year as i32;
        let month = month as u32;
        let day = day as u32;
        let racecourse = Racecourse::from_jravan_id(racecourse as i32);

        Self {
            date: NaiveDate::from_ymd(year, month, day),
            racecourse: racecourse,
        }
    }

    pub fn to_date_racecourse_id(&self) -> i64 {
        (self.date.year() as i64) * 1000000
            + (self.date.month() as i64) * 10000
            + (self.date.day() as i64) * 100
            + self.racecourse as i64
    }

    pub fn to_race(self, race_num: i32) -> Race {
        Race {
            date: self.date,
            racecourse: self.racecourse,
            race_num: race_num,
        }
    }
}

impl GetPath for DateRacecourse {
    fn get_dir_path(&self) -> std::path::PathBuf {
        PathBuf::new()
            .join(self.racecourse.to_string())
            .join(format!("{}", self.date.format("%Y-%m")))
    }

    fn get_data_id(&self) -> String {
        self.to_date_racecourse_id().to_string()
    }
}
