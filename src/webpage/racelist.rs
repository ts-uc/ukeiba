use crate::{common::date_racecourse::DateRacecourse};
use scraper::{Html, Selector};
use crate::db_writer::DbType;
use crate::db_writer::RaceListData;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug)]
pub struct PageRaceList {
    pub html: String,
    pub date_racecourse: DateRacecourse,
}

#[derive(Debug)]
pub struct RaceData {
    pub race_num: i32,
    pub post_time: Option<String>,
    pub change: Option<String>,
    pub race_type: Option<String>,
    pub race_name: Option<String>,
    pub surface: Option<String>,
    pub direction: Option<String>,
    pub distance: Option<i32>,
    pub weather: Option<String>,
    pub going: Option<String>,
    pub moisture: Option<f64>,
    pub horse_count: Option<i32>,
}

impl PageRaceList {
    pub fn new(html: String, date_racecourse: DateRacecourse) -> PageRaceList {
        PageRaceList {
            html: html,
            date_racecourse: date_racecourse,
        }
    }

    fn scrap(&self) -> Vec<RaceData> {
        // 当日メニューをスクレイピングし、ベクタ形式で返す
        if self.html.contains("ご指定のレース一覧の情報がありません") {
            return Vec::new();
        }

        let document = Html::parse_document(&self.html);

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
            let race_num: i32 = *&td_element_vector[0].replace("R", "").parse().unwrap();

            let racedata = RaceData {
                race_num: race_num,
                post_time: to_some_string(&td_element_vector[1]),
                change: to_some_string(&td_element_vector[2]),
                race_type: to_some_string(&td_element_vector[3]),
                race_name: to_some_string(&td_element_vector[4]),
                surface: surface,
                direction: direction,
                distance: distance,
                weather: to_some_string(&td_element_vector[6]),
                going: going,
                moisture: moisture,
                horse_count: *&td_element_vector[8].parse().ok(),
            };
            r.push(racedata);
        }
        r
    }

    pub fn db(&self) -> Vec<DbType> {
        let scrapped = self.scrap();
        let mut _dbtype_vec: Vec<DbType> = Vec::new();
        for _racedata in scrapped {
            let _dbtype = DbType::RaceList(RaceListData {
                race_id: self
                    .date_racecourse
                    .to_race(_racedata.race_num)
                    .to_race_id(),
                race_date: self.date_racecourse.date.to_string(),
                racecourse: self.date_racecourse.racecourse.to_string(),
                race_num: _racedata.race_num,
                post_time: _racedata.post_time,

                change: _racedata.change,
                race_type: _racedata.race_type,
                race_name: _racedata.race_name,
                surface: _racedata.surface,
                direction: _racedata.direction,

                distance: _racedata.distance,
                weather: _racedata.weather,
                going: _racedata.going,
                moisture: _racedata.moisture,
                horse_count: _racedata.horse_count,
            });
            _dbtype_vec.push(_dbtype);
        }
        _dbtype_vec
    }
}

fn to_some_string(arg: &str) -> Option<String> {
    if arg.is_empty() {
        None
    } else {
        Some(arg.to_string())
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
