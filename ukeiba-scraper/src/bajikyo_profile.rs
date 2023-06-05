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
    pub sire_bajikyo_id: Option<String>,
    pub dam_bajikyo_id: Option<String>,
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
        todo!()
    }
}
