use crate::common::Racecourse;

use super::*;
use anyhow::{bail, Result};
use chrono::NaiveDate;
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone)]
pub struct Page {
    pub race_date: NaiveDate,
    pub racecourse: Racecourse,
    pub race_num: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Data {
    pub race_date: NaiveDate,
    pub racecourse: Racecourse,
    pub race_num: i32,
    pub odds_win_place_show: Vec<OddsWinPlaceShow>,
    pub odds_bracket_quinella: Vec<OddsBracketQuinella>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OddsBracketQuinella {
    pub bracket_num1: i32,
    pub bracket_num2: i32,
    pub odds: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OddsWinPlaceShow {
    pub horse_num: i32,
    pub odds_win: Option<f64>,
    pub odds_place_show_min: Option<f64>,
    pub odds_place_show_max: Option<f64>,
}

impl WebPageTrait for Page {
    type Data = Data;

    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("fetched")
            .join("oddspark_odds_1")
            .join(format!(
                "{}_{}_{}.html.xz",
                self.race_date,
                self.racecourse.to_name(),
                self.race_num
            ))
    }
    fn fetch_string(&self, interval: Duration) -> Result<String> {
        let url = format!(
            "https://www.oddspark.com/keiba/Odds.do?sponsorCd=04&opTrackCd={:02}&raceDy={}&raceNb={}&viewType=0&betType=1",
            self.racecourse.to_nar_id(),
            self.race_date.format("%Y%m%d"),
            self.race_num
        );
        let got_string = get_from_url(&url, interval)?;
        if !got_string.contains("html") {
            bail!("required tag is not exist");
        }
        Ok(got_string)
    }
    fn scrap_string(&self, body: &str) -> Result<Self::Data> {
        let doc_str: String = body.nfkc().collect();
        let doc = Html::parse_document(&doc_str);
        let doc = doc.root_element();

        let selector_str = ".tb71 > tbody:nth-child(1) > tr";
        let selector = Selector::parse(selector_str).unwrap();
        let horse_count = (doc.select(&selector).count() - 1) as i32;

        let mut odds_win_place_show = Vec::new();
        for horse_num in 1..=horse_count {
            let selector_str: String = format!(
                ".tb71 > tbody:nth-child(1) > tr:nth-child({}) > td",
                horse_num + 1
            );
            let row_count = scrap_count(&doc, &selector_str).unwrap_or_default();
            let odds_win_selector = format!(
                "tr:nth-child({}) > td:nth-child({}) > span:nth-child(1)",
                horse_num + 1,
                row_count - 1
            );
            let place_show_min_selector = format!(
                "tr:nth-child({}) > td:nth-child({}) > span:nth-child(1)",
                horse_num + 1,
                row_count
            );
            let place_show_max_selector = format!(
                "tr:nth-child({}) > td:nth-child({}) > span:nth-child(1)",
                horse_num + 1,
                row_count
            );
            odds_win_place_show.push(OddsWinPlaceShow {
                horse_num: horse_num,
                odds_win: scrap_remove_tag(&doc, &odds_win_selector).and_then(|x| x.parse().ok()),
                odds_place_show_min: scrap_remove_tag(&doc, &place_show_min_selector)
                    .and_then(|x| x.parse().ok()),
                odds_place_show_max: scrap_remove_tag(&doc, &place_show_max_selector)
                    .and_then(|x| x.parse().ok()),
            });
        }
        let data = Data {
            race_date: self.race_date,
            racecourse: self.racecourse,
            race_num: self.race_num,
            odds_win_place_show: odds_win_place_show,
            odds_bracket_quinella: Vec::new(),
        };
        Ok(data)
    }
}
