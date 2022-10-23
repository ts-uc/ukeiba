use chrono::{Date, Local};
use scraper::{ElementRef, Html, Selector};

use crate::scrap::RaceData;
use crate::enums::*;

use unicode_normalization::UnicodeNormalization;

use super::CustomError;

fn get_url(date: &Date<Local>, racecourse: &Racecourse) -> String {
    format!(
        "https://www2.keiba.go.jp/KeibaWeb/TodayRaceInfo/RaceList?k_raceDate={}&k_babaCode={}",
        date.format("%Y/%m/%d"),
        racecourse.get_keibagojp_id()
    )
}

fn fetch(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    eprintln!("Fetching {:?}...", &url);
    let res = reqwest::blocking::get(url)?;
    eprintln!("Response: {:?} {}", &res.version(), &res.status());
    let body = res.text()?.to_string();
    Ok(body)
}

fn to_some_string(arg: &str) -> Option<String> {
    if arg.is_empty() {
        None
    } else {
        Some(arg.to_string())
    }
}

fn detect_class(race_name: &str, racecourse: &Racecourse) -> Option<String> {
    if race_name.contains("オープン-") {
        Some("オープン".to_string())
    } else if race_name.contains("A1-") || race_name.ends_with("A1") || race_name.contains("A1混合")
    {
        Some("A1".to_string())
    } else if race_name.contains("A2-") || race_name.ends_with("A2") {
        Some("A2".to_string())
    } else if race_name.contains("B1-") || race_name.ends_with("B1") {
        Some("B1".to_string())
    } else if race_name.contains("B2-") || race_name.ends_with("A2") {
        Some("B2".to_string())
    } else if race_name.contains("B3-") || race_name.ends_with("A3") {
        Some("B3".to_string())
    } else if race_name.contains("B4-") || race_name.ends_with("B4") {
        Some("B4".to_string())
    } else if race_name.contains("C1-") || race_name.ends_with("C1") {
        Some("C1".to_string())
    } else if race_name.contains("C2-") || race_name.ends_with("C2") {
        Some("C2".to_string())
    } else if race_name.contains("2歳A-")
        || race_name.ends_with("2歳A")
        || race_name.contains("3歳A-")
        || race_name.ends_with("3歳A")
        || race_name.contains("2歳受賞")
    {
        Some("YA".to_string())
    } else if race_name.contains("2歳B-")
        || race_name.ends_with("2歳B")
        || race_name.contains("3歳B-")
        || race_name.ends_with("3歳B")
    {
        Some("YB".to_string())
    } else if race_name.contains("2歳C-")
        || race_name.ends_with("2歳C")
        || race_name.contains("3歳C-")
        || race_name.ends_with("3歳C")
    {
        Some("YC".to_string())
    } else if race_name.contains("2歳D-")
        || race_name.ends_with("2歳D")
        || race_name.contains("3歳D-")
        || race_name.ends_with("3歳D")
        || race_name.ends_with("2歳未受賞")
    {
        Some("YD".to_string())
    } else if race_name.contains("A-") || race_name.ends_with("A") {
        Some("A".to_string())
    } else if race_name.contains("B-") || race_name.ends_with("B") {
        Some("B".to_string())
    } else if race_name.contains("C-") || race_name.ends_with("C") {
        Some("C".to_string())
    } else if race_name.contains("2歳-") || race_name.ends_with("2歳") || race_name.contains("2歳新馬") { Some("Y".to_string())
    } else {
        None
    }
}

pub fn scrap_racecard(
    date: &Date<Local>,
    racecourse: &Racecourse,
) -> Result<Vec<RaceData>, CustomError> {
    // 当日メニューをスクレイピングし、ベクタ形式で返す
    let url = get_url(date, racecourse);
    let body = fetch(&url).ok().ok_or(CustomError::FetchingError)?;

    if body.contains("ご指定のレース一覧の情報がありません") {
        return Err(CustomError::NonBusinessDay);
    }

    let document = Html::parse_document(&body);

    let selector_str = ".raceTable > table:nth-child(1) > tbody:nth-child(1) > tr.data";
    let selector = Selector::parse(selector_str).unwrap();
    let td_selector = Selector::parse("td").unwrap();

    let td_data_select = document.select(&selector);

    let mut r = vec![];

    for td_data_element in td_data_select{
        let td_element = td_data_element.select(&td_selector);
        let td_element_vector = td_element
            .map(|x| {
                x.text()
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .nfkc()
                    .collect::<String>()
            })
            .collect::<Vec<String>>();

        let racedata = RaceData {
            race: td_element_vector[0].replace("R", "").parse().unwrap(),
            posttime: to_some_string(&td_element_vector[1]),
            change: to_some_string(&td_element_vector[2]),
            racetype: to_some_string(&td_element_vector[3]),
            name: to_some_string(&td_element_vector[4]),
            class: detect_class(&td_element_vector[4], racecourse),
            corse: to_some_string(&td_element_vector[5]),
            weather: to_some_string(&td_element_vector[6]),
            going: to_some_string(&td_element_vector[7]),
            count: to_some_string(&td_element_vector[8]),
        };

        r.push(racedata);
    }

    Ok(r)
}
