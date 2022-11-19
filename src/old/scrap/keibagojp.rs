use crate::enums::*;

use crate::scrap::keibagojp_racecard::scrap_racecard;
use crate::scrap::scrap_result;
use crate::scrap::write_racecard::{insert_racecard, insert_result};

use chrono::prelude::*;
use std::thread;

pub fn scrap_keibagojp(date: Date<Local>, racecourse: &Racecourse) -> Result<(), CustomError> {
    thread::sleep(std::time::Duration::from_secs(5));
    let racecard = scrap_racecard(&date, racecourse)?;
    println!("{:?}", &racecard);
    insert_racecard(&racecard);

    for x in racecard {
        thread::sleep(std::time::Duration::from_secs(5));

        let result = match scrap_result(&x.date, &x.racecourse, &x.race) {
            Ok(x) => x,
            Err(_) => continue,
        };
        insert_result(&result);
    }

    // 当日メニューをDBに書き込む
    // 各レースをスクレイピングする
    // 各レースごとにデータをDBに書き込む
    Ok(())
}
