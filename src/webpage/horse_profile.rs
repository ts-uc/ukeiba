use super::*;
use crate::common::horse::Horse;
use crate::DbType;
use anyhow::Result;
use std::path::PathBuf;

pub struct HorseProfilePage(pub Horse);

impl WebPage for HorseProfilePage {
    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("horse_profile")
            .join(self.0.get_upper_id().to_string())
            .join(format!("horse_profile_{}.html.gz", self.0.get_horse_id()))
    }
    fn fetch(&self) -> Result<String> {
        let url = format!(
            "https://www.keiba.go.jp/KeibaWeb/DataRoom/RaceHorseInfo?k_lineageLoginCode={}&k_activeCode=1",
            self.0.get_horse_id()
        );
        get_from_url(&url)
    }
    fn scrap(&self, body: &str) -> Vec<DbType> {
        todo!()
    }
}
