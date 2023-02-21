use super::WebPage;
use crate::common::date_racecourse::DateRacecourse;
use crate::DbType;
use anyhow::Result;
use std::path::PathBuf;

pub struct RakutenRacelistPage(pub DateRacecourse);

impl WebPage for RakutenRacelistPage {
    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("rakuten_racelist")
            .join(self.0.racecourse.to_string())
            .join(format!("{}", self.0.date.format("%Y-%m")))
            .join(format!("rakuten_racelist_{}.html.gz", self.0.to_string()))
    }
    fn fetch(&self) -> Result<String> {
        todo!()
    }
    fn scrap(&self, body: &str) -> Vec<DbType> {
        todo!()
    }
}
