#![allow(unused)]
use crate::enums::Racecourse;
use chrono::{Date, Local, TimeZone};
use std::fmt;

struct Race {
    date: Date<Local>,
    racecourse: Racecourse,
    race_num: i32,
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{:02}{:02}",
            self.date.format("%Y%m%d"),
            self.racecourse as i32,
            self.race_num,
        )
    }
}

impl std::str::FromStr for Race {
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
            race_num: s[10..12].parse().unwrap(),
        })
    }
}
