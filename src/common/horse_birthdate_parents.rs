use crate::Horse;
use chrono::NaiveDate;

use super::GetPath;

#[derive(Debug)]
pub struct HorseBirthdateParents {
    pub horse: Horse,
    pub birthdate: NaiveDate,
    pub sire_name: String,
    pub dam_name: String,
}

impl GetPath for HorseBirthdateParents {
    fn get_dir_path(&self) -> std::path::PathBuf {
        self.horse.get_dir_path()
    }

    fn get_data_id(&self) -> String {
        self.horse.get_data_id()
    }
}
