use crate::{common::{ race::Race}, webpage::race::PageRace};

use super::Reader;

pub struct RaceReader(Race);

impl RaceReader {
    pub fn new(race: Race) -> Self {
        Self(race)
    }

    pub fn get(&self, is_force_fetch: bool, is_save: bool) -> PageRace {
        PageRace::new(self.get_string(is_force_fetch, is_save).unwrap(), self.0)
    }
}

impl Reader for RaceReader {
    fn get_url(&self) -> String {
        format!(
            "https://www.keiba.go.jp/KeibaWeb/TodayRaceInfo/DebaTable?k_raceDate={}&k_raceNo={}&k_babaCode={}",
            self.0.date.format("%Y/%m/%d"),
            self.0.race_num,
            self.0.racecourse.get_keibagojp_id()
        )
    }

    fn get_file_dir_path(&self) -> std::path::PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("race")
            .join(self.0.racecourse.to_string())
            .join(format!("{}", self.0.date.format("%Y-%m")))
    }

    fn get_file_path(&self) -> std::path::PathBuf {
        self.get_file_dir_path()
            .join(format!("race_{}.html.gz", self.0.to_string()))
    }
}
