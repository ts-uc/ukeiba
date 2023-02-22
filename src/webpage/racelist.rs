use super::*;
use crate::common::date_racecourse::DateRacecourse;
use crate::db_writer::{DateRacecourses, DbType, Races};
use anyhow::Result;
use scraper::{Html, Selector};
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

pub struct RacelistPage(pub DateRacecourse);

impl WebPage for RacelistPage {
    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("racelist")
            .join(self.0.racecourse.to_string())
            .join(format!("{}", self.0.date.format("%Y-%m")))
            .join(format!("racelist_{}.html.gz", self.0.to_string()))
    }
    fn fetch(&self) -> Result<String> {
        let url = format!(
            "https://www2.keiba.go.jp/KeibaWeb/TodayRaceInfo/RaceList?k_raceDate={}&k_babaCode={}",
            self.0.date.format("%Y/%m/%d"),
            self.0.racecourse.get_keibagojp_id()
        );
        get_from_url(&url)
    }
    fn scrap(&self, body: &str) -> Vec<DbType> {
        // 当日メニューをスクレイピングし、ベクタ形式で返す
        if body.contains("ご指定のレース一覧の情報がありません") {
            return Vec::new();
        }

        let document: String = body.nfkc().collect();
        let document = Html::parse_document(&document);
        let row_selector = ".raceTable > table:nth-child(1) > tbody:nth-child(1) > tr.data";
        let row_selector = Selector::parse(row_selector).unwrap();
        let column_selector = "td";
        let column_selector = Selector::parse(column_selector).unwrap();

        let scrapped = scrap_grid(&document, &row_selector, &column_selector);

        let mut data = Vec::new();

        let date_racecourse = DateRacecourses {
            date_racecourse_id: self.0.to_date_racecourse_id(),
            race_date: self.0.date.to_string(),
            racecourse: self.0.racecourse.to_japanese(),
            kai: None,
            nichi: None,
        };

        data.push(DbType::DateRacecourse(date_racecourse));

        for scrapped_row in scrapped {
            let race_num: i32 = scrapped_row[0].replace("R", "").parse().unwrap();
            let race_id = self.0.to_race(race_num).to_race_id();

            let race_num: i32 = scrapped_row[0].replace("R", "").parse().unwrap();

            let racedata = Races {
                race_id: race_id,
                date_racecourse_id: self.0.to_date_racecourse_id(),
                race_num: race_num,
                post_time: Some(scrapped_row[1].clone()).filter(|s| !s.is_empty()),
                change: Some(scrapped_row[2].clone()).filter(|s| !s.is_empty()),
                race_type: Some(scrapped_row[3].clone()).filter(|s| !s.is_empty()),
                race_name: Some(scrapped_row[4].clone()).filter(|s| !s.is_empty()),
                surface: detect_surface(&scrapped_row[5]),
                direction: detect_direction(&scrapped_row[5]),
                distance: detect_num(&scrapped_row[5]),
                weather: Some(scrapped_row[6].clone()).filter(|s| !s.is_empty()),
                going: detect_going(&scrapped_row[7]),
                moisture: detect_num(&scrapped_row[7]),
                horse_count: scrapped_row[8].parse().ok(),
                race_age: None,
                race_horse_type: None,
                race_weight_type: None,
                race_sub_title: None,
            };
            data.push(DbType::RaceList(racedata));
        }
        data
    }
}
