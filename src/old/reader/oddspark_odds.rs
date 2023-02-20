use crate::{common::{ race::Race}, webpage::{oddspark_odds::PageOddsparkOdds}};

use super::Reader;

pub struct OddsparkOddsReader(Race);

impl OddsparkOddsReader {
    pub fn new(race: Race) -> Self {
        Self(race)
    }

    pub fn get(&self, is_force_fetch: bool, is_save: bool) -> PageOddsparkOdds {
        PageOddsparkOdds::new(self.get_string(is_force_fetch, is_save).unwrap(), self.0)
    }
}

impl Reader for OddsparkOddsReader {
    fn get_url(&self) -> String {
        format!(
            "https://www.oddspark.com/keiba/Odds.do?raceDy={}&opTrackCd={:02}&sponsorCd=04&betType=1&viewType=0&raceNb={}",
            self.0.date.format("%Y%m%d"),
            self.0.racecourse.get_oddspark_id(),
            self.0.race_num,
        )
    }

    fn get_file_dir_path(&self) -> std::path::PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("oddspark_odds")
            .join(self.0.racecourse.to_string())
            .join(format!("{}", self.0.date.format("%Y-%m")))
    }

    fn get_file_path(&self) -> std::path::PathBuf {
        self.get_file_dir_path()
            .join(format!("oddspark_odds_{}.html.gz", self.0.to_string()))
    }
}
