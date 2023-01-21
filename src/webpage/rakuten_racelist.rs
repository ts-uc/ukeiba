use super::*;
use crate::common::date_racecourse::DateRacecourse;
use crate::db_writer::{DbType, Races};
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

        let selector_str = ".tb71 > tbody:nth-child(1) > tr";
        let selector = Selector::parse(selector_str).unwrap();

        let horse_count = (document.select(&selector).count() - 1) as i32;

        println!("{}", horse_count);

        // 当日メニューをスクレイピングし、ベクタ形式で返す
        let data: Vec<DbType> = Vec::new();
        data
    }
}

fn scrap(html: &Html, selector_str: &str) -> Option<String> {
    let selector = Selector::parse(&selector_str).unwrap();
    let text = scrap_text(&html, &selector);
    text.filter(|s| !s.is_empty())
}
