use crate::{common::date_racecourse::DateRacecourse, webpage::{rakuten_racelist::PageRakutenRaceList}};

use super::Reader;

pub struct RakutenRaceListReader(DateRacecourse);

impl RakutenRaceListReader {
    pub fn new(date_racecourse: DateRacecourse) -> Self {
        Self(date_racecourse)
    }

    pub fn get(&self, is_force_fetch: bool, is_save: bool) -> PageRakutenRaceList {
        PageRakutenRaceList::new(self.get_string(is_force_fetch, is_save).unwrap(), self.0)
    }
}

impl Reader for RakutenRaceListReader {
    fn get_url(&self) -> String {
        format!(
            "https://keiba.rakuten.co.jp/race_card/list/RACEID/{}{:02}00000000",
            self.0.date.format("%Y%m%d"),
            self.0.racecourse.get_keibagojp_id()
        )
    }

    fn get_file_dir_path(&self) -> std::path::PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("rakuten_racelist")
            .join(self.0.racecourse.to_string())
            .join(format!("{}", self.0.date.format("%Y-%m")))
    }

    fn get_file_path(&self) -> std::path::PathBuf {
        self.get_file_dir_path()
            .join(format!("rakuten_racelist_{}.html.gz", self.0.to_string()))
    }
}
