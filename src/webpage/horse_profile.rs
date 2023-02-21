use super::WebPage;
use crate::common::horse::Horse;
use crate::DbType;
use anyhow::Result;
use std::path::PathBuf;

pub struct HorseProfilePage(pub Horse);

impl WebPage for HorseProfilePage {
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
