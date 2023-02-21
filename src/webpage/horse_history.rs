use super::*;
use crate::common::horse::Horse;
use crate::DbType;
use anyhow::Result;
use std::path::PathBuf;

pub struct HorseHistoryPage(pub Horse);

impl WebPage for HorseHistoryPage {
    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("horse_history")
            .join(self.0.get_upper_id().to_string())
            .join(format!("horse_history_{}.html.gz", self.0.get_horse_id()))
    }
    fn fetch(&self) -> Result<String> {
        let url = format!(
            "https://www2.keiba.go.jp/KeibaWeb/DataRoom/HorseMarkInfo?k_lineageLoginCode={}",
            self.0.get_horse_id()
        );
        get_from_url(&url)
    }
    fn scrap(&self, body: &str) -> Vec<DbType> {
        todo!()
    }
}
