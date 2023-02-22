use super::*;
use crate::common::race::Race;
use crate::db_writer::DbType;
use crate::db_writer::RaceHorses;
use anyhow::Result;
use scraper::{Html, Selector};
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

pub struct OddsparkOddsPage(pub Race);

impl WebPage for OddsparkOddsPage {
    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("oddspark_odds")
            .join(self.0.racecourse.to_string())
            .join(format!("{}", self.0.date.format("%Y-%m")))
            .join(format!("oddspark_odds_{}.html.gz", self.0.to_string()))
    }
    fn fetch(&self) -> Result<String> {
        let url = format!(
            "https://www.oddspark.com/keiba/Odds.do?raceDy={}&opTrackCd={:02}&sponsorCd=04&betType=1&viewType=0&raceNb={}",
            self.0.date.format("%Y%m%d"),
            self.0.racecourse.get_oddspark_id(),
            self.0.race_num,
        );
        get_from_url(&url)
    }
    fn scrap(&self, body: &str) -> Vec<DbType> {
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
                bracket_num: None,
                horse_name: None,
                horse_sex: None,
                horse_age: None,
                horse_id: None,
                jockey_name: None,
                jockey_id: None,
                trainer_name: None,
                trainer_id: None,
                change: None,
                owner_name: None,
                weight_mark: None,
                weight_to_carry: None,
                horse_weight: None,
                horse_weight_delta: None,
                arrival: None,
                finish_time: None,
                margin_time: None,
                margin: None,
                last_3f: None,
                win_fav: None,
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
                prize: None,
            };
            data.push(DbType::OddsparkOdds(data_))
        }
        // 当日メニューをスクレイピングし、ベクタ形式で返す
        data
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
