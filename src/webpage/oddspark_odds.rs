use super::WebPage;
use crate::common::race::Race;
use crate::DbType;
use anyhow::Result;
use std::path::PathBuf;

pub struct OddsparkOddsPage(pub Race);

impl WebPage for OddsparkOddsPage {
    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("oddspark_odds")
            .join(self.0.racecourse.to_string())
            .join(format!("{}", self.0.date.format("%Y-%m")))
            .join(format!("oddspark_odds_{}.html.gz", self.0.to_string()))
    }
    fn fetch(&self) -> Result<String> {
        todo!()
    }
    fn scrap(&self, body: &str) -> Vec<DbType> {
        todo!()
    }
}
