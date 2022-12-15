use super::WebPage;
use crate::common::date_racecourse::DateRacecourse;
use crate::common::race;
use crate::common::race::Race;
use crate::enums::CustomError;
use rusqlite::params;
use rusqlite::Connection;
use scraper::{Html, Selector};
use core::panic;
use std::fs::File;
use std::io::Write;
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

impl WebPage for PageRaceList {
    fn save_to_file(&self) -> () {
        let b = self.html.as_bytes();
        let filename = dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("racelist")
            .join(format!(
                "racelist_{}.html",
                self.date_racecourse.to_string()
            ));
        let mut file = File::create(filename).unwrap();
        file.write_all(b).unwrap();
    }

    fn save_to_db(&self) -> () {
        let racelist = self.scrap_racecard();
        println!("{:?}", racelist);
        // match racelist {
        //     Ok(races) => update_db(&races),
        //     Err(_) => return,
        // }
    }
}

impl PageRaceList {
    pub fn scrap_racecard(&self) -> Result<Vec<RaceData>, CustomError> {
        // 当日メニューをスクレイピングし、ベクタ形式で返す
        if self.html.contains("ご指定のレース一覧の情報がありません") {
            return Err(CustomError::NonBusinessDay);
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
        println!("{:?}", r);

        Ok(r)
    }

    pub fn update_db(races: &Vec<RaceData>) {
        todo!()
        // let path = "./ukeiba.db";
        // let conn = Connection::open(&path).unwrap();
    
        // for racedata in races {
        //     conn.execute(
        //         "REPLACE  INTO race (
        //             race_id, race_date, racecource, race_num, post_time,
        //             change, race_type, race_name, surface,
        //             direction, distance, weather, going, moisture,
        //             horse_count) 
        //             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
        //         params![
        //             &racedata.race.to_string().parse::<i32>().unwrap(),
        //             &racedata.race.gen_date(),
        //             &racedata.race.racecourse.get_name(),
        //             &racedata.race.race_num,
        //             &racedata.post_time,
        //             //
        //             &racedata.change,
        //             &racedata.race_type,
        //             &racedata.race_name,
        //             &racedata.surface,
        //             //
        //             &racedata.distance,
        //             &racedata.direction,
        //             &racedata.weather,
        //             &racedata.going,
        //             &racedata.moisture,
        //             //
        //             &racedata.horse_count,
        //         ],
        //     )
        //     .unwrap();
        // }
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
