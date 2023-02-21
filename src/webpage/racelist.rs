use super::WebPage;
use crate::common::date_racecourse::DateRacecourse;
use crate::DbType;
use anyhow::Result;
use std::path::PathBuf;

pub struct RacelistPage(pub DateRacecourse);

impl WebPage for RacelistPage {
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
