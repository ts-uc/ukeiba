use chrono::{Date, Local};
use scraper::{ElementRef, Html, Selector};

use crate::enums::*;
use crate::scrap::RaceResult;

use unicode_normalization::UnicodeNormalization;
use url::Url;

fn get_url(date: &Date<Local>, racecourse: &Racecourse, race: &i32) -> String {
    format!(
        "https://www2.keiba.go.jp/KeibaWeb/TodayRaceInfo/RaceMarkTable?k_raceDate={}&k_raceNo={}&k_babaCode={}",
        date.format("%Y/%m/%d"),
        race,
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

fn get_req_param_num<T: std::str::FromStr>(element_ref: &ElementRef, selector: &Selector) -> Result<T, CustomError> {
    let id_url = element_ref
        .select(selector)
        .next()
        .ok_or(CustomError::Error)?
        .value()
        .attr("href")
        .ok_or(CustomError::Error)?
        .trim();
    let id_url = Url::parse(&("http://example.com".to_string() + id_url))
        .ok()
        .ok_or(CustomError::Error)?;
    let mut id_pairs = id_url.query_pairs();
    let (_, id) = id_pairs.next().ok_or(CustomError::Error)?;
    let id = id.parse::<T>().ok().ok_or(CustomError::Error)?;
    Ok(id)
}

pub fn scrap_result(
    date: &Date<Local>,
    racecourse: &Racecourse,
    race: &i32,
) -> Result<Vec<RaceResult>, CustomError> {
    // 当日メニューをスクレイピングし、ベクタ形式で返す
    let url = get_url(date, racecourse, race);
    let body = fetch(&url).ok().ok_or(CustomError::FetchingError)?;

    //println!("{}", body);
    if body.contains("ご指定のレース成績の情報がありません") {
        return Err(CustomError::RaceResultInfoIsNotExist);
    }

    let document = Html::parse_document(&body);

    let selector_str = "table.bs:nth-child(11) > tbody:nth-child(1) > tr:nth-child(1) > td:nth-child(1) > table:nth-child(1) > tbody:nth-child(1) > tr[bgcolor=\"#FFFFFF\"]";
    let selector = Selector::parse(selector_str).unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let horse_selector = Selector::parse("span:nth-child(1) > a:nth-child(1)").unwrap();
    let jockey_selector = Selector::parse("td:nth-child(8) > a:nth-child(1)").unwrap();
    let trainer_selector = Selector::parse("td:nth-child(9) > a:nth-child(1)").unwrap();

    let td_data_select = document.select(&selector);

    let mut r = vec![];

    for td_data_element in td_data_select {
        let td_element = td_data_element.select(&td_selector);
        let td_element_vector = td_element
            .clone()
            .map(|x| {
                x.text()
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .nfkc()
                    .collect::<String>()
            })
            .collect::<Vec<String>>();

        let horse_id: Option<i64> = get_req_param_num(&td_data_element, &horse_selector).ok();
        let jockey_id: Option<i32> = get_req_param_num(&td_data_element, &jockey_selector).ok();
        let trainer_id: Option<i32> = get_req_param_num(&td_data_element, &trainer_selector).ok();
        let sex_age : Vec<&str> = td_element_vector[5].split_whitespace().collect();

        let race_result = RaceResult{
            date: date.clone(),
            racecourse: racecourse.clone(),
            race: race.clone(),
            horse_num: td_element_vector[2].parse().unwrap(),
            bracket_num: td_element_vector[1].parse().ok(),
            arrival: td_element_vector[0].parse().ok(),
            horse_name: to_some_string(&td_element_vector[3]),
            horse_id: horse_id,
            horse_affiliation: to_some_string(&td_element_vector[4]),
            horse_sex: to_some_string(&sex_age[0]),
            horse_age: sex_age[1].parse().ok(),
            weight_to_carry: td_element_vector[6].parse().ok(),
            jockey: to_some_string(&td_element_vector[7]),
            jockey_id: jockey_id,
            trainer: to_some_string(&td_element_vector[8]),
            trainer_id: trainer_id,
            horse_weight: td_element_vector[9].parse().ok(),
            horse_weight_delta: td_element_vector[10].parse().ok(),
            finish: to_some_string(&td_element_vector[11]),
            margin: to_some_string(&td_element_vector[12]),
            three_furlongs: td_element_vector[13].parse().ok(),
            win_fav: td_element_vector[14].parse().ok(),
        };
        r.push(race_result);
    }

    //     let (surface, direction, distance) = detect_corse(&td_element_vector[5]);
    //     let (going, moisture) = detect_going(&td_element_vector[7]);

    //     let racedata = RaceData {
    //         date: date.clone(),
    //         racecourse: racecourse.clone(),
    //         race: *&td_element_vector[0].replace("R", "").parse().unwrap(),
    //         posttime: to_some_string(&td_element_vector[1]),
    //         change: to_some_string(&td_element_vector[2]),
    //         racetype: to_some_string(&td_element_vector[3]),
    //         name: to_some_string(&td_element_vector[4]),
    //         class: detect_class(&td_element_vector[4], racecourse),
    //         surface: surface,
    //         direction: direction,
    //         distance: distance,
    //         weather: to_some_string(&td_element_vector[6]),
    //         going: going,
    //         moisture: moisture,
    //         count: *&td_element_vector[8].parse().ok(),
    //     };

    //     r.push(racedata);
    // }

    Ok(r)
}
