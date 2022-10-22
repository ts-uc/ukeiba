mod keibagojp;
mod keibagojp_racecard;
mod write_racecard;

use crate::Racecourse;

use chrono::prelude::*;

use self::keibagojp::scrap_keibagojp;

#[derive(Debug)]
pub struct RaceData{
    race: i32,
    posttime: Option<String>,
    change: Option<String>,
    racetype: Option<String>,
    name: Option<String>,
    corse: Option<String>,
    weather: Option<String>,
    going: Option<String>,
    count: Option<String>
}

//指定した日付・競馬場のデータをWebサイトから取得し、sqliteに書き込む
pub fn scrap(date:Date<Local>, racecourse:Racecourse){
    scrap_keibagojp(date, racecourse)
}