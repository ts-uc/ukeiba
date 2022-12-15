#![allow(unused)]
use crate::{enums::Racecourse, common::date_racecourse, reader::racelist::RaceList};
use chrono::{Date, Local, TimeZone};
use std::fmt;

#[derive ( Debug)]
pub struct DateRacecourse {
    pub date: Date<Local>,
    pub racecourse: Racecourse,
}

impl fmt::Display for DateRacecourse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{:02}",
            self.date.format("%Y%m%d"),
            self.racecourse as i32,
        )
    }
}

impl std::str::FromStr for DateRacecourse {
    type Err = std::convert::Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            date: Local.ymd(
                s[0..4].parse().unwrap(),
                s[4..6].parse().unwrap(),
                s[6..8].parse().unwrap(),
            ),
            racecourse: s[8..10].parse::<Racecourse>().unwrap(),
        })
    }
}

impl DateRacecourse{
    pub fn make_racelist_reader(self) -> RaceList{
        RaceList::new(self)
    }
}
