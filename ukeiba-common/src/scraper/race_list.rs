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
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Data {
    pub race_date: NaiveDate,
    pub racecourse: Racecourse,
    pub is_race_date: bool,
    pub race_list: Vec<RaceList>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RaceList {
    pub race_num: i32,
    pub post_time: Option<String>,
    pub race_type: Option<String>,
}

impl WebPageTrait for Page {
    type Data = Data;

    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("fetched")
            .join("race_list")
            .join(format!(
                "{}_{}.html.xz",
                self.race_date,
                self.racecourse.to_name(),
            ))
    }
    fn fetch_string(&self, interval: Duration) -> Result<String> {
        let url = format!(
            "https://www.keiba.go.jp/KeibaWeb/TodayRaceInfo/RaceList?k_raceDate={}&k_babaCode={}",
            self.race_date.format("%Y/%m/%d"),
            self.racecourse.to_nar_id()
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

        let is_race_date = !doc_str.contains("ご指定のレースの情報がありません");

        let mut race_list = Vec::new();

        for element in doc.select(
            &Selector::parse(".raceTable > table:nth-child(1) > tbody:nth-child(1) > tr.data")
                .unwrap(),
        ) {
            race_list.push(RaceList {
                race_num: scrap(&element, "td:nth-child(1)")
                    .unwrap_or_default()
                    .replace("R", "")
                    .parse()
                    .unwrap_or_default(),
                post_time: scrap(&element, "td:nth-child(2)"),
                race_type: match scrap(&element, "td:nth-child(4)").as_deref() {
                    Some("重賞") => Some("重賞".to_string()),
                    Some("準重賞") => Some("準重賞".to_string()),
                    Some("特別") => Some("特別".to_string()),
                    None => Some("一般".to_string()),
                    Some(_) => None,
                },
            });
        }

        Ok(Data {
            race_date: self.race_date,
            racecourse: self.racecourse,
            is_race_date: is_race_date,
            race_list: race_list,
        })
    }
}
