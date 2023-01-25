use super::*;
use crate::common::date_racecourse::DateRacecourse;
use crate::db_writer::DateRacecourses;
use crate::db_writer::DbType;
use scraper::{Html, Selector};
use unicode_normalization::UnicodeNormalization;

#[derive(Debug)]
pub struct PageRakutenRaceList {
    pub html: String,
    pub date_racecourse: DateRacecourse,
}

impl PageRakutenRaceList {
    pub fn new(html: String, date_racecourse: DateRacecourse) -> PageRakutenRaceList {
        PageRakutenRaceList {
            html: html,
            date_racecourse: date_racecourse,
        }
    }

    pub fn db(&self) -> Vec<DbType> {
        let document: String = self.html.nfkc().collect();
        let document = Html::parse_document(&document);

        let title = scrap(&document, "div.headline > h2:nth-child(1)");

        let kai: Option<i32> = title
            .clone()
            .and_then(|f| detect_kai(&f))
            .and_then(|f| f.parse().ok());
        let nichi: Option<i32> = title
            .and_then(|f| detect_nichi(&f))
            .and_then(|f| f.parse().ok());

        // 当日メニューをスクレイピングし、ベクタ形式で返す
        let mut data: Vec<DbType> = Vec::new();
        let date_racecourse = DateRacecourses {
            date_racecourse_id: self.date_racecourse.to_date_racecourse_id(),
            race_date: self.date_racecourse.date.to_string(),
            racecourse: self.date_racecourse.racecourse.to_japanese(),
            kai: kai,
            nichi: nichi,
        };

        data.push(DbType::RakutenDateRacecourse(date_racecourse));
        data
    }
}

fn scrap(html: &Html, selector_str: &str) -> Option<String> {
    let selector = Selector::parse(&selector_str).unwrap();
    let text = scrap_text(&html, &selector);
    text.filter(|s| !s.is_empty())
}

fn detect_kai(str: &str) -> Option<String> {
    Some(Regex::new(r"第(\d+?)回").unwrap().captures(str)?[1].to_string()).filter(|s| !s.is_empty())
}

fn detect_nichi(str: &str) -> Option<String> {
    Some(Regex::new(r"第(\d+?)日").unwrap().captures(str)?[1].to_string()).filter(|s| !s.is_empty())
}
