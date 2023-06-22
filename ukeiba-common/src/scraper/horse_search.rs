use super::*;
use crate::common::HorseBelong;
use anyhow::{bail, Result};
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Page {
    pub page_num: i32,
    pub horse_name: String,
    pub horse_belong: HorseBelong,
    pub birth_year: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]

pub struct Data {
    pub page_num: i32,
    pub horse_name: String,
    pub horse_belong: HorseBelong,
    pub birth_year: i32,
    pub hits: i32,
    pub hits_all: i32,
    pub horse_nar_ids: Vec<i64>,
}

impl WebPageTrait for Page {
    type Data = Data;

    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("fetched")
            .join("horse_search")
            .join(format!(
                "{}_{}_{}_{}.html.xz",
                self.page_num, self.horse_name, self.horse_belong as i32, self.birth_year
            ))
    }
    fn fetch_string(&self, interval: Duration) -> Result<String> {
        let url = format!(
            "https://www.keiba.go.jp/KeibaWeb/DataRoom/RaceHorseList?k_pageNum={}&k_horseName={}&k_horseNameCondition=start&k_horsebelong={}&k_birthYear={}&k_fatherHorse=&k_fatherHorseCondition=start&k_motherHorse=&k_motherHorseCondition=start&k_activeCode=1&k_dataKind=*",
            self.page_num, self.horse_name, self.horse_belong as i32, self.birth_year
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

        let mut data = Vec::new();

        for element in doc.select(
            &Selector::parse("table.databasesearch_table:nth-child(3) > tbody:nth-child(2) > tr")
                .unwrap(),
        ) {
            data.push(
                scrap_link(&element, "td:nth-child(2) > a:nth-child(1)")
                    .and_then(|s| get_query(&s, "k_lineageLoginCode")?.parse().ok())
                    .unwrap_or_default(),
            );
        }

        let hits: i32 = scrap(
            &doc,
            ".searchconditionindex > li:nth-child(1) > span:nth-child(1)",
        )
        .and_then(|s| s.parse().ok())
        .unwrap_or_default();

        Ok(Data {
            page_num: self.page_num,
            horse_name: self.horse_name.clone(),
            horse_belong: self.horse_belong,
            birth_year: self.birth_year,
            hits: hits,
            hits_all: scrap(
                &doc,
                ".searchconditionindex > li:nth-child(1) > span:nth-child(2)",
            )
            .and_then(|s| s.replace("件", "").parse().ok())
            .unwrap_or(hits),
            horse_nar_ids: data,
        })
    }
}
