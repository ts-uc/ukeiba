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
    pub kai: Option<i32>,
    pub nichi: Option<i32>,
}

impl WebPageTrait for Page {
    type Data = Data;

    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("fetched")
            .join("rakuten_racelist")
            .join(format!(
                "{}_{}.html.xz",
                self.race_date,
                self.racecourse.to_name(),
            ))
    }
    fn fetch_string(&self, interval: Duration) -> Result<String> {
        let url = format!(
            "https://keiba.rakuten.co.jp/race_card/list/RACEID/{}{:02}00000000",
            self.race_date.format("%Y%m%d"),
            self.racecourse.to_nar_id(),
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

        let (kai, nichi) =
            split_kai_nichi(&scrap(&doc, "div.headline > h2:nth-child(1)").unwrap_or_default());

        Ok(Data {
            race_date: self.race_date,
            racecourse: self.racecourse,
            kai: kai,
            nichi: nichi,
        })
    }
}

fn split_kai_nichi(raw: &str) -> (Option<i32>, Option<i32>) {
    let re = regex::Regex::new(r"第(\d+)回.+第(\d+)日").unwrap();

    if let Some(captures) = re.captures(raw) {
        let group1 = captures
            .get(1)
            .map(|m| m.as_str().to_string())
            .and_then(|x| x.parse().ok());

        let group2 = captures
            .get(2)
            .map(|m| m.as_str().to_string())
            .and_then(|x| x.parse().ok());

        return (group1, group2);
    }

    (None, None)
}
