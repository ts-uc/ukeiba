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
    pub belong: HorseBelong,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]

pub struct Data {
    pub page_num: i32,
    pub belong: HorseBelong,
    pub hits: i32,
    pub data: Vec<DataRow>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]

pub struct DataRow {
    pub jockey_nar_id: i64,
    pub jockey_name: String,
}

impl WebPageTrait for Page {
    type Data = Data;

    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("fetched")
            .join("jockey_search")
            .join(format!("{}_{}.html.xz", self.page_num, self.belong as i32))
    }
    fn fetch_string(&self, interval: Duration) -> Result<String> {
        let url = format!(
            "https://www.keiba.go.jp/KeibaWeb/DataRoom/RiderList?k_pageNum={}&k_name=&k_nameCondition=start&k_genneki_flag=*&k_syozoku={}&k_sei=",
            self.page_num, self.belong as i32
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
            &Selector::parse("li.DRListsItem > table:nth-child(1) > tbody:nth-child(2) > tr")
                .unwrap(),
        ) {
            data.push(DataRow {
                jockey_name: scrap(&element, "td:nth-child(2) > a:nth-child(1)")
                    .map(|s| remove_whitespace(&s))
                    .unwrap_or_default(),
                jockey_nar_id: scrap_link(&element, "td:nth-child(2) > a:nth-child(1)")
                    .and_then(|s| get_query(&s, "k_riderLicenseNo")?.parse().ok())
                    .unwrap_or_default(),
            });
        }

        Ok(Data {
            page_num: self.page_num,
            belong: self.belong,
            hits: scrap(
                &doc,
                ".searchconditionindex > li:nth-child(1) > span:nth-child(1)",
            )
            .and_then(|s| s.parse().ok())
            .unwrap_or_default(),
            data: data,
        })
    }
}
