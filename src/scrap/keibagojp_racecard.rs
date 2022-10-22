use chrono::{Date, Local};
use scraper::{ElementRef, Html, Selector};

use crate::scrap::RaceData;
use crate::Racecourse;

use unicode_normalization::UnicodeNormalization;

impl Racecourse {
    fn get_babacode(&self) -> i32 {
        match self {
            Racecourse::Obihiro => 3,
            Racecourse::Morioka => 10,
            Racecourse::Urawa => 18,
            Racecourse::Kanazawa => 22,
            Racecourse::Kasamatsu => 23,
            Racecourse::Nagoya => 24,
            Racecourse::Kochi => 31,
            Racecourse::Saga => 32,
            _ => todo!(),
        }
    }
}

fn get_url(date: &Date<Local>, racecourse: &Racecourse) -> String {
    format!(
        "https://www2.keiba.go.jp/KeibaWeb/TodayRaceInfo/RaceList?k_raceDate={}&k_babaCode={}",
        date.format("%Y/%m/%d"),
        racecourse.get_babacode()
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

pub fn scrap_racecard(
    date: &Date<Local>,
    racecourse: &Racecourse,
) -> Result<Vec<RaceData>, Box<dyn std::error::Error>> {
    // 当日メニューをスクレイピングし、ベクタ形式で返す
    let url = get_url(date, racecourse);
    let body = fetch(&url)?;
    let document = Html::parse_document(&body);

    let selector_str = ".raceTable > table:nth-child(1) > tbody:nth-child(1)";
    let selector = Selector::parse(selector_str).unwrap();
    let tr_data_selector = Selector::parse("tr.data").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    let tbody = document.select(&selector).next().unwrap();

    let td_data_element = tbody.select(&tr_data_selector);
    let r = td_data_element
        .map(|y| {
            let td_element = y.select(&td_selector);
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

            RaceData {
                race: td_element_vector[0].replace("R", "").parse().unwrap(),
                posttime: to_some_string(&td_element_vector[1]),
                change: to_some_string(&td_element_vector[2]),
                racetype: to_some_string(&td_element_vector[3]),
                name: to_some_string(&td_element_vector[4]),
                corse: to_some_string(&td_element_vector[5]),
                weather: to_some_string(&td_element_vector[6]),
                going: to_some_string(&td_element_vector[7]),
                count: to_some_string(&td_element_vector[8]),
            }
        })
        .collect::<Vec<RaceData>>();

    Ok(r)
}
