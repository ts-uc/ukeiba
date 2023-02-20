use crate::{common::horse::Horse, webpage::horse_profile::PageHorseProfile};

use super::Reader;

pub struct HorseProfileReader(Horse);

impl HorseProfileReader {
    pub fn new(horse: Horse) -> Self {
        Self(horse)
    }

    pub fn get(&self, is_force_fetch: bool, is_save: bool) -> PageHorseProfile {
        PageHorseProfile::new(self.get_string(is_force_fetch, is_save).unwrap(), self.0)
    }
}

impl Reader for HorseProfileReader {
    fn get_url(&self) -> String {
        format!(
            "https://www.keiba.go.jp/KeibaWeb/DataRoom/RaceHorseInfo?k_lineageLoginCode={}&k_activeCode=1",
            self.0.get_horse_id()
        )
    }

    fn get_file_dir_path(&self) -> std::path::PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("horse_profile")
            .join(self.0.get_upper_id().to_string())
    }

    fn get_file_path(&self) -> std::path::PathBuf {
        self.get_file_dir_path()
            .join(format!("horse_profile_{}.html.gz", self.0.get_horse_id()))
    }
}
