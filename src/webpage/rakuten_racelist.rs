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
        // 当日メニューをスクレイピングし、ベクタ形式で返す
        let data: Vec<DbType> = Vec::new();
        data
    }
}
