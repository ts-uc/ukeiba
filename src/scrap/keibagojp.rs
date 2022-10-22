use crate::Racecourse;

use crate::scrap::keibagojp_racecard::scrap_racecard;

use chrono::prelude::*;



pub fn scrap_keibagojp(date:Date<Local>, racecourse:Racecourse){
    let racecard = scrap_racecard(date, racecourse);
    // 当日メニューをDBに書き込む
    // 各レースをスクレイピングする
    // 各レースごとにデータをDBに書き込む
    todo!()
 }