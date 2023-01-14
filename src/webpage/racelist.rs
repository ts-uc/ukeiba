use crate::common::date_racecourse::DateRacecourse;
use crate::db_writer::DbType;
use crate::db_writer::Races;
use scraper::{Html, Selector};
use crate::webpage::grid_scrapper;
use crate::webpage::detect_going;

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

    pub fn db(&self) -> Vec<DbType> {
        // 当日メニューをスクレイピングし、ベクタ形式で返す
        if self.html.contains("ご指定のレース一覧の情報がありません") {
            return Vec::new();
        }

        let document = Html::parse_document(&self.html);
        let row_selector = ".raceTable > table:nth-child(1) > tbody:nth-child(1) > tr.data";
        let row_selector = Selector::parse(row_selector).unwrap();
        let column_selector = "td";
        let column_selector = Selector::parse(column_selector).unwrap();

        let scrapped = grid_scrapper(&document, &row_selector, &column_selector);

        let mut data = Vec::new();

        for scrapped_row in scrapped {
            let race_num: i32 = scrapped_row[0].replace("R", "").parse().unwrap();
            let race_id = self.date_racecourse.to_race(race_num).to_race_id();

            let (surface, direction, distance) = detect_corse(&scrapped_row[5]);
            let (going, moisture) = detect_going(&scrapped_row[7]);
            let race_num: i32 = scrapped_row[0].replace("R", "").parse().unwrap();

            let racedata = Races {
                race_id: race_id,
                race_date: self.date_racecourse.date.to_string(),
                racecourse: self.date_racecourse.racecourse.to_string(),
                race_num: race_num,
                post_time: Some(scrapped_row[1].clone()).filter(|s| !s.is_empty()),
                change: Some(scrapped_row[2].clone()).filter(|s| !s.is_empty()),
                race_type: Some(scrapped_row[3].clone()).filter(|s| !s.is_empty()),
                race_name: Some(scrapped_row[4].clone()).filter(|s| !s.is_empty()),
                surface: surface,
                direction: direction,
                distance: distance,
                weather: Some(scrapped_row[6].clone()).filter(|s| !s.is_empty()),
                going: going,
                moisture: moisture,
                horse_count: scrapped_row[8].parse().ok(),
            };
            data.push(DbType::RaceList(racedata));
        }
        data
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
