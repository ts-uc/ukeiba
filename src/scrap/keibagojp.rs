use crate::enums::*;

use crate::scrap::keibagojp_racecard::scrap_racecard;
use crate::scrap::write_racecard::insert_racecard;

use chrono::prelude::*;
use super::CustomError;



pub fn scrap_keibagojp(date:Date<Local>, racecourse:&Racecourse) -> Result<(), CustomError>{
    let racecard = scrap_racecard(&date, racecourse)?;
    println!("{:?}", &racecard);
    insert_racecard(&date, racecourse, &racecard);

    // 当日メニューをDBに書き込む
    // 各レースをスクレイピングする
    // 各レースごとにデータをDBに書き込む
    Ok(())
 }