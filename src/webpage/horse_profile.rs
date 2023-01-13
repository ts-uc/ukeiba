use crate::common::horse::Horse;
use crate::db_writer::Db;
use scraper::{Html, Selector};
use crate::db_writer::DbType;
use unicode_normalization::UnicodeNormalization;
use url::Url;

#[derive(Debug)]
pub struct PageHorseProfile {
    pub html: String,
    pub horse: Horse,
}

impl PageHorseProfile {
    pub fn new(html: String, horse: Horse) -> Self {
        Self {
            html: html,
            horse: horse,
        }
    }

    pub fn db(&self) -> Db {
        Db::new(Vec::new())
    }
}

