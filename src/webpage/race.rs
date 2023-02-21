use super::WebPage;
use crate::common::race::Race;
use crate::DbType;
use anyhow::Result;
use std::path::PathBuf;

pub struct RacePage(pub Race);

impl WebPage for RacePage {
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
