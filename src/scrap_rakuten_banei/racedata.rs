use super::Race;
use super::Racecourse;
use chrono::prelude::*;

use scraper::Html;
use scraper::Selector;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug)]
pub struct RaceDataBanei {
    date: Date<Local>,
    racecourse: Racecourse,
    num: i32,
    weather: String,
    track_condition: String,
    post_time: String,
    name: String,
    class: String,
    horse_condition: String,
}

pub trait RaceDataBaneiTrait {
    fn scrap_str(&self, selectors: &str) -> String;
    fn scrap_weather(&self) -> String;
    fn scrap_track_condition(&self) -> String;
    fn scrap_post_time(&self) -> String;
    fn scrap_name(&self) -> String;
    fn scrap_class(&self) -> String;
    fn horse_condition(&self) -> String;
    fn scrap_race_data(&self, race: Race) -> RaceDataBanei;
}

impl RaceDataBaneiTrait for Html {
    fn scrap_str(&self, selectors: &str) -> String {
        let weatcher_selector = Selector::parse(selectors).unwrap();
        self.select(&weatcher_selector)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()[0]
            .nfkc()
            .collect::<String>()
    }

    fn scrap_weather(&self) -> String {
        self.scrap_str(
            "ul.trackState:nth-child(2) > li:nth-child(2) > dl:nth-child(1) > dd:nth-child(2)",
        )
    }

    fn scrap_track_condition(&self) -> String {
        self.scrap_str(
            "ul.trackState:nth-child(2) > li:nth-child(2) > dl:nth-child(1) > dd:nth-child(4)",
        )
    }

    fn scrap_post_time(&self) -> String {
        self.scrap_str(
            "ul.trackState:nth-child(2) > li:nth-child(2) > dl:nth-child(1) > dd:nth-child(6)",
        )
    }

    fn scrap_name(&self) -> String {
        self.scrap_str(".raceNote > h2:nth-child(3)")
    }

    fn scrap_class(&self) -> String {
        self.scrap_name()
    }
    
    fn horse_condition(&self) -> String {
        self.scrap_str(".horseCondition > li:nth-child(1)")
    }

    fn scrap_race_data(&self, race: Race) -> RaceDataBanei {
        RaceDataBanei {
            date: race.date,
            racecourse: race.racecourse,
            num: race.num,
            weather: self.scrap_weather(),
            track_condition: self.scrap_track_condition(),
            post_time: self.scrap_post_time(),
            name: self.scrap_name(),
            class: self.scrap_class(),
            horse_condition: self.horse_condition(),
        }
    }
}
