use anyhow::Result;
use regex::Regex;
use scraper::{Html, Selector};
use std::path::{Path, PathBuf};
pub mod bajikyo_search;
pub mod horse_history;
pub mod horse_profile;
pub mod oddspark_odds;
pub mod race;
pub mod racelist;
pub mod rakuten_racelist;
use self::{
    bajikyo_search::BajikyoSearchPage, horse_history::HorseHistoryPage,
    horse_profile::HorseProfilePage, oddspark_odds::OddsparkOddsPage, race::RacePage,
    racelist::RacelistPage, rakuten_racelist::RakutenRacelistPage,
};
use crate::db_writer::DbType;

pub trait WebPage {
    fn get_path(&self) -> PathBuf;
    fn fetch(&self) -> Result<String>;
    fn scrap(&self, body: &str) -> Vec<DbType>;
}
pub enum WebPageType {
    BajikyoSearch(BajikyoSearchPage),
    HorseHistory(HorseHistoryPage),
    HorseProfile(HorseProfilePage),
    OddsparkOdds(OddsparkOddsPage),
    Race(RacePage),
    Racelist(RacelistPage),
    RakutenRacelist(RakutenRacelistPage),
}

impl WebPageType {
    fn get_path(&self) -> PathBuf {
        match self {
            Self::BajikyoSearch(x) => x.get_path(),
            Self::HorseHistory(x) => x.get_path(),
            Self::HorseProfile(x) => x.get_path(),
            Self::OddsparkOdds(x) => x.get_path(),
            Self::Race(x) => x.get_path(),
            Self::Racelist(x) => x.get_path(),
            Self::RakutenRacelist(x) => x.get_path(),
        }
    }
    fn fetch(&self) -> Result<String> {
        match self {
            Self::BajikyoSearch(x) => x.fetch(),
            Self::HorseHistory(x) => x.fetch(),
            Self::HorseProfile(x) => x.fetch(),
            Self::OddsparkOdds(x) => x.fetch(),
            Self::Race(x) => x.fetch(),
            Self::Racelist(x) => x.fetch(),
            Self::RakutenRacelist(x) => x.fetch(),
        }
    }
    fn scrap(&self, body: &str) -> Vec<DbType> {
        match self {
            Self::BajikyoSearch(x) => x.scrap(body),
            Self::HorseHistory(x) => x.scrap(body),
            Self::HorseProfile(x) => x.scrap(body),
            Self::OddsparkOdds(x) => x.scrap(body),
            Self::Race(x) => x.scrap(body),
            Self::Racelist(x) => x.scrap(body),
            Self::RakutenRacelist(x) => x.scrap(body),
        }
    }
}
