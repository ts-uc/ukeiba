use super::*;
use anyhow::{bail, Context, Result};
use chrono::NaiveDate;
use scraper::Html;
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone)]
pub struct Page {
    pub horse_nar_id: i64,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub horse_name: String,
    pub horse_sex: String,
    pub horse_status: String,
    pub horse_type: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub sire_name: Option<String>,
    pub dam_name: Option<String>,
}

impl WebPageTrait for Page {
    type Data = Data;

    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("fetched")
            .join("horse_profile")
            .join(format!("{}.html.xz", self.horse_nar_id.to_string()))
    }
    fn fetch_string(&self, interval: Duration) -> Result<String> {
        let url = format!(
            "https://www.keiba.go.jp/KeibaWeb/DataRoom/RaceHorseInfo?k_lineageLoginCode={}&k_activeCode=1",
            self.horse_nar_id
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

        Ok(Data {
            horse_name: scrap(&doc, ".odd_title").context("essential error")?,
            horse_sex: scrap(&doc, ".sex").context("essential error")?,
            horse_status: scrap(&doc, ".horseinfo > li:nth-child(3) > div:nth-child(1)")
                .context("essential error")?,
            horse_type: scrap(&doc, ".horseinfo > li:nth-child(4) > div:nth-child(1)"),
            birthdate: scrap(
                &doc,
                ".horse_info_table > tbody:nth-child(2) > tr:nth-child(1) > td:nth-child(2)",
            ).and_then(|s| NaiveDate::parse_from_str(&s, "%Y.%m.%d生").ok()),
            sire_name: scrap(&doc, ".fathername").map(|s| split_bracket(&s).2.to_string()),
            dam_name: scrap(&doc, ".pedigree > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child(3) > td:nth-child(2)")
            .map(|s| split_bracket(&s).2.to_string()),
        })
    }
}
