use super::WebPage;
use crate::common::horse::Horse;
use crate::DbType;
use anyhow::Result;
use std::path::PathBuf;

pub struct HorseHistoryPage(pub Horse);

impl WebPage for HorseHistoryPage {
    fn get_path(&self) -> PathBuf {
        todo!()
    }
    fn fetch(&self) -> Result<String> {
        todo!()
    }
    fn scrap(&self, body: &str) -> Vec<DbType> {
        todo!()
    }
}
