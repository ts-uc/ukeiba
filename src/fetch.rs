use chrono::{Date, Local};
use crate::enums::*;

fn fetch(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    eprintln!("Fetching {:?}...", &url);
    let res = reqwest::blocking::get(url)?;
    eprintln!("Response: {:?} {}", &res.version(), &res.status());
    let body = res.text()?.to_string();
    Ok(body)
}

pub fn fetch_racelist(date: &Date<Local>, racecourse: &Racecourse) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://www2.keiba.go.jp/KeibaWeb/TodayRaceInfo/RaceList?k_raceDate={}&k_babaCode={}",
        date.format("%Y/%m/%d"),
        racecourse.get_keibagojp_id()
    );
    fetch(&url)
}

pub fn fetch_race(date: &Date<Local>, racecourse: &Racecourse, race: &i32) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://www2.keiba.go.jp/KeibaWeb/TodayRaceInfo/RaceMarkTable?k_raceDate={}&k_raceNo={}&k_babaCode={}",
        date.format("%Y/%m/%d"),
        race,
        racecourse.get_keibagojp_id()
    );
    fetch(&url)
}
