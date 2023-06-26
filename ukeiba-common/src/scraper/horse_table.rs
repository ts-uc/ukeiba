use crate::common::Racecourse;

use super::*;
use anyhow::{bail, Result};
use chrono::NaiveDate;
use scraper::Html;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::{alloc::handle_alloc_error, path::PathBuf};
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone)]
pub struct Page {
    pub race_date: NaiveDate,
    pub racecourse: Racecourse,
    pub race_num: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]

pub struct Data {
    pub race_sub_title: Option<String>,
    pub race_title: String,
    pub registered_horse_count: i32,
    pub data: Vec<DataRow>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]

pub struct DataRow {
    pub horse_num: i32,
    pub horse_nar_id: i64,
    pub horse_sex: Option<String>,
    pub horse_age: Option<i32>,
    pub jockey_nar_id: Option<i32>,
    pub horse_weight_mark: Option<String>,
    pub weight_to_carry: Option<i32>,
    pub trainer_nar_id: Option<i32>,
    pub owner_name: Option<String>,
    pub horse_weight: Option<i32>,
    pub horse_change: Option<String>,
}

impl WebPageTrait for Page {
    type Data = Data;
    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("fetched")
            .join("horse_table")
            .join(format!(
                "{}_{}_{}.html.xz",
                self.race_date,
                self.racecourse.to_name(),
                self.race_num
            ))
    }
    fn fetch_string(&self, interval: Duration) -> Result<String> {
        let url = format!(
            "https://www.keiba.go.jp/KeibaWeb/TodayRaceInfo/DebaTable?k_raceDate={}&k_raceNo={}&k_babaCode={}",
            self.race_date.format("%Y/%m/%d"),
            self.race_num,
            self.racecourse.to_nar_id()
        );
        let got_string = get_from_url(&url, interval)?;
        if !got_string.contains("html") {
            bail!("required tag is not exist");
        }
        Ok(got_string)
    }
    fn scrap_string(&self, body: &str) -> Result<Self::Data> {
        {
            let doc: String = body.nfkc().collect();
            let doc = Html::parse_document(&doc);
            let doc = doc.root_element();

            let selector_str = ".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr";
            let horse_count =
                ((scrap_count(&doc, selector_str).unwrap_or_default() - 2) / 5) as i32;

            let mut data = Vec::new();

            for horse_num in 1..=horse_count {
                let selector_str = format!("tr.tBorder:nth-child({}) > td", horse_num * 5 - 2);
                let row_count = scrap_count(&doc, &selector_str).unwrap_or_default();

                let selector_str = format!(
                    "tr.tBorder:nth-child({}) > td:nth-child({}) > a:nth-child(1)",
                    horse_num * 5 - 2,
                    row_count - 8
                );

                let horse_nar_id = scrap_link(&doc, &selector_str).and_then(|x| {
                    get_query(&x, "k_lineageLoginCode").and_then(|x| x.parse::<i64>().ok())
                });

                let selector_str = format!(
                    ".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(1)",
                    horse_num * 5 - 1,
                );
                let (sex, age) = scrap_remove_tag(&doc, &selector_str)
                    .map(|x| split_sexage(&x))
                    .unwrap_or_default();

                let selector_str = format!(
                    "tr.tBorder:nth-child({}) > td:nth-child({}) > a:nth-child(1)",
                    horse_num * 5 - 2,
                    row_count - 7
                );
                let jockey_nar_id = scrap_link(&doc, &selector_str).and_then(|x| {
                    get_query(&x, "k_riderLicenseNo").and_then(|x| x.parse::<i32>().ok())
                });

                let selector_str = format!(
                    ".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(4)",
                    horse_num * 5 - 1,
                );
                let (weight_mark, weight_to_carry) = scrap(&doc, &selector_str)
                    .map(|x| split_weight(&x))
                    .unwrap_or_default();

                let selector_str = format!(
                    ".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(2) > a:nth-child(1)",
                    horse_num * 5,
                );
                let trainer_nar_id = scrap_link(&doc, &selector_str).and_then(|x| {
                    get_query(&x, "k_trainerLicenseNo").and_then(|x| x.parse::<i32>().ok())
                });

                let selector_str = format!(
                    ".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(2)",
                    horse_num * 5 + 1,
                );
                let owner_name = scrap(&doc, &selector_str);

                let selector_str = format!(
                    ".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(3)",
                    horse_num * 5,
                );
                let horse_weight = scrap_remove_tag(&doc, &selector_str)
                    .and_then(|x| split_bracket(&x).0.parse::<i32>().ok());

                let selector_str = format!(
                    ".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(3)",
                    horse_num * 5 + 2,
                );
                let horse_change = scrap(&doc, &selector_str);

                data.push(DataRow {
                    horse_num: horse_num,
                    horse_nar_id: horse_nar_id.unwrap_or_default(),
                    horse_sex: sex,
                    horse_age: age,
                    jockey_nar_id: jockey_nar_id,
                    horse_weight_mark: weight_mark,
                    weight_to_carry: weight_to_carry,
                    trainer_nar_id: trainer_nar_id,
                    owner_name: owner_name,
                    horse_weight: horse_weight,
                    horse_change: horse_change,
                    ..Default::default()
                });
            }

            Ok(Data {
                race_sub_title: scrap(&doc, ".subTitle"),
                race_title: scrap(&doc, ".raceTitle > h3:nth-child(4)").unwrap_or_default(),
                registered_horse_count: horse_count,
                data: data,
            })
        }
    }
}

fn split_sexage(raw: &str) -> (Option<String>, Option<i32>) {
    let re = regex::Regex::new(r"^\s*(牡|牝|セン)(\d)\s*$").unwrap();

    if let Some(captures) = re.captures(raw) {
        let group1 = captures.get(1).map(|m| m.as_str().to_string());
        let group2 = captures
            .get(2)
            .map(|m| m.as_str().to_string())
            .and_then(|x| x.parse().ok());

        return (group1, group2);
    }

    (None, None)
}

fn split_weight(raw: &str) -> (Option<String>, Option<i32>) {
    let re = regex::Regex::new(r"(★|▲|△|☆)?\s*(\d+)\s*\d+-\d+-\d+-\d+").unwrap();

    if let Some(captures) = re.captures(raw) {
        let group1 = captures.get(1).map(|m| m.as_str().to_string());
        let group2 = captures
            .get(2)
            .map(|m| m.as_str().to_string())
            .and_then(|x| x.parse().ok());

        return (group1, group2);
    }

    (None, None)
}
