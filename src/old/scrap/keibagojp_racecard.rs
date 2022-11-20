use chrono::{Date, Local};
use scraper::{Html, Selector};

use crate::enums::*;
use crate::common::RaceData;



use unicode_normalization::UnicodeNormalization;

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
    if *racecourse == Racecourse::Obihiro {
        if race_name.contains("オープン-") {
            Some("オープン".to_string())
        } else if race_name.contains("A1-")
            || race_name.ends_with("A1")
            || race_name.contains("A1混合")
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
        } else if race_name.contains("2歳-")
            || race_name.ends_with("2歳")
            || race_name.contains("2歳新馬")
        {
            Some("Y".to_string())
        } else {
            None
        }
    } else {
        None
    }
}

fn detect_corse(course: &str) -> (Option<String>, Option<String>, Option<i32>) {
    if course == "" {
        return (None, None, None);
    };
    let surface = if course.contains("芝") {
        Some("芝".to_string())
    } else {
        Some("ダ".to_string())
    };

    let direction = if course.contains("右") {
        Some("右".to_string())
    } else if course.contains("左") {
        Some("左".to_string())
    } else if course.contains("直") {
        Some("直".to_string())
    } else {
        None
    };

    let distance = course
        .replace("芝", "")
        .replace("右", "")
        .replace("左", "")
        .replace("直", "")
        .replace("m", "")
        .parse()
        .ok();

    (surface, direction, distance)
}

fn detect_going(raw_going: &str) -> (Option<String>, Option<f64>) {
    if raw_going == "" {
        return (None, None);
    };

    let going = match raw_going {
        "良" => Some("良".to_string()),
        "稍重" => Some("稍重".to_string()),
        "重" => Some("重".to_string()),
        "不良" => Some("不良".to_string()),
        _ => None,
    };

    let moisture: Option<f64> = raw_going.parse().ok();

    (going, moisture)
}


pub fn scrap_racecard(
    date: &Date<Local>,
    racecourse: &Racecourse,
    body: &str,
) -> Result<Vec<RaceData>, CustomError> {
    // 当日メニューをスクレイピングし、ベクタ形式で返す
    if body.contains("ご指定のレース一覧の情報がありません") {
        return Err(CustomError::NonBusinessDay);
    }

    let document = Html::parse_document(&body);

    let selector_str = ".raceTable > table:nth-child(1) > tbody:nth-child(1) > tr.data";
    let selector = Selector::parse(selector_str).unwrap();
    let td_selector = Selector::parse("td").unwrap();

    let td_data_select = document.select(&selector);

    let mut r = vec![];

    for td_data_element in td_data_select {
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
        
        let (surface, direction, distance) = detect_corse(&td_element_vector[5]);
        let (going, moisture) = detect_going(&td_element_vector[7]);

        let racedata = RaceData {
            date: date.clone(),
            racecourse: racecourse.clone(),
            race: *&td_element_vector[0].replace("R", "").parse().unwrap(),
            posttime: to_some_string(&td_element_vector[1]),
            change: to_some_string(&td_element_vector[2]),
            racetype: to_some_string(&td_element_vector[3]),
            name: to_some_string(&td_element_vector[4]),
            class: detect_class(&td_element_vector[4], racecourse),
            surface: surface,
            direction: direction,
            distance: distance,
            weather: to_some_string(&td_element_vector[6]),
            going: going,
            moisture: moisture,
            count: *&td_element_vector[8].parse().ok(),
        };

        r.push(racedata);
    }

    Ok(r)
}