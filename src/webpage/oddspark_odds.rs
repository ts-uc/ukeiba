use super::*;
use crate::common::race::Race;
use crate::db_writer::DbType;
use crate::db_writer::RaceHorses;
use anyhow::Result;
use scraper::{Html, Selector};
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone)]
pub struct OddsparkOddsPage(pub Race);

impl WebPageTrait for OddsparkOddsPage {
    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("oddspark_odds")
            .join(self.0.racecourse.to_string())
            .join(format!("{}", self.0.date.format("%Y-%m")))
            .join(format!("oddspark_odds_{}.html.gz", self.0.to_string()))
    }
    fn fetch_string(&self) -> Result<String> {
        let url = format!(
            "https://www.oddspark.com/keiba/Odds.do?raceDy={}&opTrackCd={:02}&sponsorCd=04&betType=1&viewType=0&raceNb={}",
            self.0.date.format("%Y%m%d"),
            self.0.racecourse.get_oddspark_id(),
            self.0.race_num,
        );
        get_from_url(&url)
    }
    fn scrap(&self, body: &str) -> Result<Vec<DbType>> {
        let document: String = body.nfkc().collect();
        let document = Html::parse_document(&document);

        let selector_str = ".tb71 > tbody:nth-child(1) > tr";
        let selector = Selector::parse(selector_str).unwrap();

        let horse_count = (document.select(&selector).count() - 1) as i32;

        let mut data: Vec<DbType> = Vec::new();

        for horse_num in 1..=horse_count {
            let (_, bracket_num_index) = calc_wakuban(horse_count, horse_num);
            let data_ = RaceHorses {
                race_horse_id: self.0.to_race_horse(horse_num).to_racehorse_id(),
                race_id: self.0.to_race_id(),
                horse_num: horse_num,
                win_odds: scrap(
                    &document,
                    &format!(
                        "tr:nth-child({}) > td:nth-child({}) > span:nth-child(1)",
                        horse_num + 1,
                        4 - bracket_num_index
                    ),
                ),
                place_odds_min: scrap(
                    &document,
                    &format!(
                        "tr:nth-child({}) > td:nth-child({}) > span:nth-child(1)",
                        horse_num + 1,
                        5 - bracket_num_index
                    ),
                ),
                place_odds_max: scrap(
                    &document,
                    &format!(
                        "tr:nth-child({}) > td:nth-child({}) > span:nth-child(2)",
                        horse_num + 1,
                        5 - bracket_num_index
                    ),
                ),
                ..Default::default()
            };
            data.push(DbType::OddsparkOdds(data_))
        }
        // 当日メニューをスクレイピングし、ベクタ形式で返す
        Ok(data)
    }
}

fn calc_wakuban(horse_count: i32, horse_num: i32) -> (i32, i32) {
    if horse_count <= 8 {
        (horse_num, 0)
    } else {
        let base_num = 16 - horse_count;
        if horse_num <= base_num {
            (horse_num, 0)
        } else {
            let foo = horse_num - base_num + 1;
            (base_num + foo / 2, foo % 2)
        }
    }
}
