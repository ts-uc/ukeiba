use super::WebPage;
use crate::common::race::Race;
use crate::DbType;
use anyhow::Result;
use std::path::PathBuf;

pub struct RacePage(pub Race);

impl WebPage for RacePage {
    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("race")
            .join(self.0.racecourse.to_string())
            .join(format!("{}", self.0.date.format("%Y-%m")))
            .join(format!("race_{}.html.gz", self.0.to_string()))
    }
    fn fetch(&self) -> Result<String> {
        todo!()
    }
    fn scrap(&self, body: &str) -> Vec<DbType> {
        todo!()
    }
}
