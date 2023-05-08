use super::*;
use anyhow::{bail, Result};
use chrono::NaiveDate;
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone)]
pub struct Page {
    pub jockey_nar_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub name: String,
    pub kana: String,
    pub sex: String,
    pub status: String,
    pub affiliation: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub first_run: Option<NaiveDate>,
    pub first_win: Option<NaiveDate>,
}

impl WebPageTrait for Page {
    type Data = Data;

    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("fetched")
            .join("jockey_profile")
            .join(format!("{}.html.xz", self.jockey_nar_id.to_string()))
    }
    fn fetch_string(&self, interval: Duration) -> Result<String> {
        let url = format!(
            "https://www.keiba.go.jp/KeibaWeb/DataRoom/RiderMark?k_riderLicenseNo={}",
            self.jockey_nar_id
        );
        let got_string = get_from_url(&url, interval)?;
        if !got_string.contains("html") {
            bail!("required tag is not exist");
        }
        Ok(got_string)
    }
    fn scrap_string(&self, body: &str) -> Result<Data> {
        let doc: String = body.nfkc().collect();
        let doc = Html::parse_document(&doc);
        let doc = doc.root_element();

        Ok(Data {
            name: scrap(&doc, ".horseinfo > li:nth-child(1) > h4:nth-child(1)")
                .map(|s| remove_whitespace(&s))
                .unwrap_or_default(),
            kana: scrap(&doc, "h4.mini")
                .map(|s| split_bracket(&s).1.to_string())
                .unwrap_or_default(),
            sex: scrap(&doc, ".sex").unwrap_or_default(),
            status: scrap(&doc, ".horseinfo > li:nth-child(4) > div:nth-child(1)")
                .unwrap_or_default(),
            affiliation: scrap(
                &doc,
                ".trainerinfo > tbody:nth-child(1) > tr:nth-child(1) > td:nth-child(2)",
            ),
            birthdate: scrap(
                &doc,
                ".trainerinfo > tbody:nth-child(1) > tr:nth-child(3) > td:nth-child(2)",
            )
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y/%m/%d").ok()),
            first_run: scrap(
                &doc,
                ".trainerinfo > tbody:nth-child(1) > tr:nth-child(4) > td:nth-child(2)",
            )
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y/%m/%d").ok()),
            first_win: scrap(
                &doc,
                ".trainerinfo > tbody:nth-child(1) > tr:nth-child(5) > td:nth-child(2)",
            )
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y/%m/%d").ok()),
        })
    }
}
