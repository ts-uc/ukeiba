use super::*;
use anyhow::{bail, Result};
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
    pub sire_bajikyo_id: Option<String>,
    pub dam_bajikyo_id: Option<String>,
    pub bms_bajikyo_id: Option<String>,
}

impl WebPageTrait for Page {
    type Data = Data;

    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("fetched")
            .join("bajikyo_pedigree")
            .join(format!("{}.html.xz", self.horse_bajikyo_id))
    }
    fn fetch_string(&self, interval: Duration) -> Result<String> {
        let url = format!(
            "https://www.bajikyo.or.jp/renkei.php?pageno=206&assoc=1&hno={}",
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

        let sire_id_selector = ".ho_00_01 > div:nth-child(1) > p:nth-child(1) > span:nth-child(1) > strong:nth-child(1) > a:nth-child(1)";
        let dam_id_selector = ".ho_00_02 > div:nth-child(1) > p:nth-child(1) > span:nth-child(1) > strong:nth-child(1) > a:nth-child(1)";
        let bms_id_selector = ".ho_01_03 > div:nth-child(1) > p:nth-child(1) > span:nth-child(1) > strong:nth-child(1) > a:nth-child(1)";
        let sire_id: Option<String> =
            scrap_link(&doc, sire_id_selector).and_then(|s| get_query(&s, "hno")?.parse().ok());
        let dam_id: Option<String> =
            scrap_link(&doc, dam_id_selector).and_then(|s| get_query(&s, "hno")?.parse().ok());
        let bms_id: Option<String> =
            scrap_link(&doc, bms_id_selector).and_then(|s| get_query(&s, "hno")?.parse().ok());

        Ok(Data {
            horse_bajikyo_id: self.horse_bajikyo_id.clone(),
            sire_bajikyo_id: sire_id,
            dam_bajikyo_id: dam_id,
            bms_bajikyo_id: bms_id,
        })
    }
}
