use crate::{common::date_racecourse::DateRacecourse, webpage::racelist::PageRaceList};

use super::Reader;

pub struct RaceListReader(DateRacecourse);

impl RaceListReader {
    pub fn new(date_racecourse: DateRacecourse) -> Self {
        Self(date_racecourse)
    }

    pub fn get(&self, is_force_fetch: bool, is_save: bool) -> PageRaceList {
        PageRaceList::new(self.get_string(is_force_fetch, is_save).unwrap(), self.0)
    }
}

impl Reader for RaceListReader {
    fn get_url(&self) -> String {
        format!(
            "https://www2.keiba.go.jp/KeibaWeb/TodayRaceInfo/RaceList?k_raceDate={}&k_babaCode={}",
            self.0.date.format("%Y/%m/%d"),
            self.0.racecourse.get_keibagojp_id()
        )
    }

    fn get_file_dir_path(&self) -> std::path::PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("racelist")
            .join(self.0.racecourse.to_string())
            .join(format!("{}", self.0.date.format("%Y-%m")))
    }

    fn get_file_path(&self) -> std::path::PathBuf {
        self.get_file_dir_path()
            .join(format!("racelist_{}.html.gz", self.0.to_string()))
    }
}
