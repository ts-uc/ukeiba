use super::WebPage;
use crate::common::race::Race;
use crate::DbType;
use anyhow::Result;
use std::path::PathBuf;

pub struct OddsparkOddsPage(pub Race);

impl WebPage for OddsparkOddsPage {
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
