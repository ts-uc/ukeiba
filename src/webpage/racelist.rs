use super::*;
use crate::common::date_racecourse::DateRacecourse;
use crate::DbType;
use anyhow::Result;
use std::path::PathBuf;

pub struct RacelistPage(pub DateRacecourse);

impl WebPage for RacelistPage {
    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("racelist")
            .join(self.0.racecourse.to_string())
            .join(format!("{}", self.0.date.format("%Y-%m")))
            .join(format!("racelist_{}.html.gz", self.0.to_string()))
    }
    fn fetch(&self) -> Result<String> {
        let url = format!(
            "https://www2.keiba.go.jp/KeibaWeb/TodayRaceInfo/RaceList?k_raceDate={}&k_babaCode={}",
            self.0.date.format("%Y/%m/%d"),
            self.0.racecourse.get_keibagojp_id()
        );
        get_from_url(&url)
    }
    fn scrap(&self, body: &str) -> Vec<DbType> {
        todo!()
    }
}
