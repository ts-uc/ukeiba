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
    pub post_time: Option<String>,
    pub post_time_change: Option<bool>,
    pub race_sub_title: Option<String>,
    pub race_title: String,
    pub race_detail: RaceDetail,
    pub race_prize: RacePrize,
    pub registered_horse_count: i32,
    pub data: Vec<DataRow>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RaceDetail {
    pub surface: Option<String>,
    pub distance: Option<String>,
    pub direction: Option<String>,
    pub weather: Option<String>,
    pub going: Option<String>,
    pub race_breed: Option<String>,
    pub race_age: Option<String>,
    pub race_weight_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RacePrize {
    pub prize1: Option<i32>,
    pub prize2: Option<i32>,
    pub prize3: Option<i32>,
    pub prize4: Option<i32>,
    pub prize5: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataRow {
    pub horse_num: i32,
    pub bracket_num: Option<i32>,
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
                    "tr.tBorder:nth-child({}) > td:nth-child(1)",
                    (horse_num - (11 - row_count)) * 5 - 2,
                );
                let bracket_num = scrap(&doc, &selector_str).and_then(|x| x.parse::<i32>().ok());

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
                    bracket_num: bracket_num,
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
                });
            }

            let (post_time, post_time_change) =
                scrap(&doc, ".raceCard > div:nth-child(1) > h4:nth-child(1)")
                    .map(|x| split_post_time(&x))
                    .unwrap_or_default();

            Ok(Data {
                race_date: self.race_date,
                racecourse: self.racecourse,
                race_num: self.race_num,
                post_time: post_time,
                post_time_change: post_time_change,
                race_sub_title: scrap(&doc, ".subTitle"),
                race_title: scrap(&doc, ".raceTitle > h3:nth-child(4)").unwrap_or_default(),
                race_detail: scrap(&doc, "ul.dataArea:nth-child(5) > li:nth-child(1)")
                    .map(|x| split_race_detail(&x))
                    .unwrap_or_default(),
                race_prize: scrap(&doc, "ul.dataArea:nth-child(5) > li:nth-child(2)")
                    .map(|x| split_prize(&x))
                    .unwrap_or_default(),
                registered_horse_count: horse_count,
                data: data,
            })
        }
    }
}

fn split_post_time(raw: &str) -> (Option<String>, Option<bool>) {
    let re = regex::Regex::new(r"(\d+:\d+)発走(\(変更\))?$").unwrap();

    if let Some(captures) = re.captures(raw) {
        let group1 = captures.get(1).map(|m| m.as_str().to_string());
        let group2 = Some(captures.get(2).is_some());

        return (group1, group2);
    }

    (None, None)
}

fn split_sexage(raw: &str) -> (Option<String>, Option<i32>) {
    let re = regex::Regex::new(r"^\s*(牡|牝|セン)(\d+)\s*$").unwrap();

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

fn split_race_detail(raw: &str) -> RaceDetail {
    let re = regex::Regex::new(
        r"^(\S+)\s+(\S+)m\((\S+)\)\s*(天候:(\S*))?\s*(馬場:(\S*))?\s+(\S+)\s+(\S+)\s+(\S+)\s+\S+$",
    )
    .unwrap();

    if let Some(captures) = re.captures(raw) {
        return RaceDetail {
            surface: captures.get(1).map(|m| m.as_str().to_string()),
            distance: captures.get(2).map(|m| m.as_str().to_string()),
            direction: captures.get(3).map(|m| m.as_str().to_string()),
            weather: captures.get(5).map(|m| m.as_str().to_string()),
            going: captures.get(7).map(|m| m.as_str().to_string()),
            race_breed: captures.get(8).map(|m| m.as_str().to_string()),
            race_age: captures.get(9).map(|m| m.as_str().to_string()),
            race_weight_type: captures.get(10).map(|m| m.as_str().to_string()),
        };
    }

    RaceDetail {
        ..Default::default()
    }
}

fn split_prize(raw: &str) -> RacePrize {
    let re = regex::Regex::new(
        r"1着([\d,]+)円\s*(2着([\d,]+)円)?\s*(3着([\d,]+)円)?\s*(4着([\d,]+)円)?\s*(5着([\d,]+)円)?",
    )
    .unwrap();

    if let Some(captures) = re.captures(raw) {
        return RacePrize {
            prize1: captures
                .get(1)
                .map(|m| m.as_str().to_string())
                .and_then(|s| s.replace(",", "").parse().ok()),
            prize2: captures
                .get(3)
                .map(|m| m.as_str().to_string())
                .and_then(|s| s.replace(",", "").parse().ok()),
            prize3: captures
                .get(5)
                .map(|m| m.as_str().to_string())
                .and_then(|s| s.replace(",", "").parse().ok()),
            prize4: captures
                .get(7)
                .map(|m| m.as_str().to_string())
                .and_then(|s| s.replace(",", "").parse().ok()),
            prize5: captures
                .get(9)
                .map(|m| m.as_str().to_string())
                .and_then(|s| s.replace(",", "").parse().ok()),
        };
    }

    RacePrize {
        ..Default::default()
    }
}
