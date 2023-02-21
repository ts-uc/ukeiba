use super::*;
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
        let url = format!(
            "https://www.oddspark.com/keiba/Odds.do?raceDy={}&opTrackCd={:02}&sponsorCd=04&betType=1&viewType=0&raceNb={}",
            self.0.date.format("%Y%m%d"),
            self.0.racecourse.get_oddspark_id(),
            self.0.race_num,
        );
        get_from_url(&url)
    }
    fn scrap(&self, body: &str) -> Vec<DbType> {
        todo!()
    }
}
