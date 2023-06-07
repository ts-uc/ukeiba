use super::*;
use anyhow::{bail, Context, Result};
use chrono::NaiveDate;
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone)]
pub struct Page {
    pub horse_bajikyo_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Data {
    pub horse_bajikyo_id: String,
    pub horse_bajikyo_name: String,
    pub horse_birthdate: Option<NaiveDate>,
    pub horse_breed: Option<String>,
    pub horse_coat_color: Option<String>,
    pub horse_breeder: Option<String>,
    pub horse_breeder_address: Option<String>,
}

impl WebPageTrait for Page {
    type Data = Data;

    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("fetched")
            .join("bajikyo_profile")
            .join(format!("{}.html.xz", self.horse_bajikyo_id))
    }
    fn fetch_string(&self, interval: Duration) -> Result<String> {
        let url = format!(
            "https://www.bajikyo.or.jp/renkei.php?pageno=203&assoc=1&hno={}",
            self.horse_bajikyo_id
        );
        let got_string = get_from_url(&url, interval)?;
        if !got_string.contains("html") {
            bail!("required tag is not exist");
        }
        Ok(got_string)
    }
    fn scrap_string(&self, body: &str) -> Result<Self::Data> {
        let doc: String = body.nfkc().collect();
        let doc = Html::parse_document(&doc);
        let doc = doc.root_element();

        let inner_selector =
            Selector::parse("#form > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(2) > td:nth-child(1) > table:nth-child(2) > tbody:nth-child(1) > tr:nth-child(2) > td:nth-child(1) > table:nth-child(1) > tbody:nth-child(1)").unwrap();
        let inner_select = doc
            .select(&inner_selector)
            .next()
            .context("failed to select inner select")?;

        let horse_bajikyo_name_selector = "#form > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(1) > td:nth-child(1) > font:nth-child(1)";

        Ok(Data {
            horse_bajikyo_id: self.horse_bajikyo_id.clone(),
            horse_bajikyo_name: scrap(&doc, &horse_bajikyo_name_selector)
                .map(|x| split_bracket(&x).0.to_string())
                .unwrap_or_default(),
            horse_birthdate: scrap(&inner_select, "tr:nth-child(3) > td:nth-child(2)")
                .and_then(|s| NaiveDate::parse_from_str(&s.trim(), "%Y/%m/%d").ok()),
            horse_coat_color: scrap(&inner_select, "tr:nth-child(5) > td:nth-child(2)"),
            horse_breed: scrap(&inner_select, "tr:nth-child(6) > td:nth-child(2)"),
            horse_breeder: scrap(&inner_select, "tr:nth-child(10) > td:nth-child(2)")
                .map(|x| remove_whitespace(&x)),
            horse_breeder_address: scrap(&inner_select, "tr:nth-child(11) > td:nth-child(2)")
                .map(|x| remove_whitespace(&x)),
        })
    }
}
