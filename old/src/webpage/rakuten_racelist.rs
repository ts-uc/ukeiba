use super::*;
use crate::common::date_racecourse::DateRacecourse;
use crate::db_writer::DateRacecourses;
use crate::db_writer::DbType;
use anyhow::{bail, Result};
use scraper::Html;
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone)]
pub struct RakutenRacelistPage(pub DateRacecourse);

impl WebPageTrait for RakutenRacelistPage {
    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("rakuten_racelist")
            .join(self.0.racecourse.to_string())
            .join(format!("{}", self.0.date.format("%Y-%m")))
            .join(format!("rakuten_racelist_{}.html.xz", self.0.to_string()))
    }
    fn fetch_string(&self) -> Result<String> {
        let url = format!(
            "https://keiba.rakuten.co.jp/race_card/list/RACEID/{}{:02}00000000",
            self.0.date.format("%Y%m%d"),
            self.0.racecourse.get_keibagojp_id()
        );
        let got_string = get_from_url(&url)?;
        if !got_string.contains("html") {
            bail!("required tag is not exist");
        }
        Ok(got_string)
    }
    fn scrap(&self, body: &str) -> Result<Vec<DbType>> {
        let document: String = body.nfkc().collect();
        let document = Html::parse_document(&document);

        let title = scrap(&document, "div.headline > h2:nth-child(1)");

        if title.is_none() {
            return Ok(Vec::new());
        }

        let kai: Option<i32> = title
            .clone()
            .and_then(|f| detect_kai(&f))
            .and_then(|f| f.parse().ok());
        let nichi: Option<i32> = title
            .and_then(|f| detect_nichi(&f))
            .and_then(|f| f.parse().ok());

        // 当日メニューをスクレイピングし、ベクタ形式で返す
        let mut data: Vec<DbType> = Vec::new();
        let date_racecourse = DateRacecourses {
            date_racecourse_id: self.0.to_date_racecourse_id(),
            race_date: self.0.date.to_string(),
            racecourse: self.0.racecourse.to_japanese(),
            kai: kai,
            nichi: nichi,
        };

        data.push(DbType::RakutenRaceListHeader(date_racecourse));
        Ok(data)
    }
}

fn detect_kai(str: &str) -> Option<String> {
    Some(Regex::new(r"第(\d+?)回").unwrap().captures(str)?[1].to_string()).filter(|s| !s.is_empty())
}

fn detect_nichi(str: &str) -> Option<String> {
    Some(Regex::new(r"第(\d+?)日").unwrap().captures(str)?[1].to_string()).filter(|s| !s.is_empty())
}
