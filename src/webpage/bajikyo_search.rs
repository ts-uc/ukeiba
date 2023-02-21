use super::WebPage;
use crate::common::horse_birthdate_parents::HorseBirthdateParents;
use crate::DbType;
use anyhow::Result;
use std::path::PathBuf;

pub struct BajikyoSearchPage(pub HorseBirthdateParents);

impl WebPage for BajikyoSearchPage {
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
