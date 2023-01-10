use crate::{common::horse::Horse, webpage::horse_history::PageHorseHistory};

use super::Reader;

pub struct HorseHistoryReader(Horse);

impl HorseHistoryReader {
    pub fn new(horse: Horse) -> Self {
        Self(horse)
    }

    pub fn get(&self, is_force_fetch: bool, is_save: bool) -> PageHorseHistory {
        PageHorseHistory::new(self.get_string(is_force_fetch, is_save).unwrap(), self.0)
    }
}

impl Reader for HorseHistoryReader {
    fn get_url(&self) -> String {
        format!(
            "https://www2.keiba.go.jp/KeibaWeb/DataRoom/HorseMarkInfo?k_lineageLoginCode={}",
            self.0.get_horse_id()
        )
    }

    fn get_file_dir_path(&self) -> std::path::PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("horse_history")
            .join(self.0.get_upper_id().to_string())
    }

    fn get_file_path(&self) -> std::path::PathBuf {
        self.get_file_dir_path()
            .join(format!("horse_history_{}.html.gz", self.0.get_horse_id()))
    }
}
