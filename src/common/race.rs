use crate::enums::Racecourse;
use chrono::{NaiveDate, Datelike};
// use std::fmt;

#[derive(Debug)]
pub struct Race {
    pub date: NaiveDate,
    pub racecourse: Racecourse,
    pub race_num: i32,
}

/*
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
 */

/*/
impl std::str::FromStr for Race {
    type Err = std::convert::Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            date: NaiveDate::from_ymd(
                s[0..4].parse().unwrap(),
                s[4..6].parse().unwrap(),
                s[6..8].parse().unwrap(),
            ),
            racecourse: s[8..10].parse::<Racecourse>().unwrap(),
            race_num: s[10..12].parse().unwrap(),
        })
    }
}
*/

impl Race{
    pub fn from_race_id(race_id: i64) -> Self{
        let year = (race_id / 100000000);
        let month = (race_id / 1000000) % 100;
        let day = (race_id / 10000) % 100;
        let racecourse = (race_id / 100) % 100;
        let race_num = race_id % 100;

        let year = year as i32;
        let month = month as u32;
        let day = day as u32;
        let racecourse = Racecourse::from_jravan_id(racecourse as i32);
        let race_num = race_num as i32;

        Self{
            date: NaiveDate::from_ymd(year, month, day),
            racecourse: racecourse,
            race_num: race_num
        }
    }

    pub fn to_race_id(&self) -> i64{
        (self.date.year() as i64) * 100000000 + 
        (self.date.month() as i64) * 1000000 +
        (self.date.day() as i64) * 10000 +
        (self.racecourse as i64) * 100 +
        self.race_num as i64
    }

}