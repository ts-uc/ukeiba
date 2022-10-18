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
    fn scrap_race_data(&self, race: Race) -> RaceDataBanei;
}

impl RaceDataBaneiTrait for Html {
    fn scrap_race_data(&self, race: Race)-> RaceDataBanei {
        let track_main_state = scrap_str(".raceNote > h2:nth-child(3)", &self);

        RaceDataBanei {
            date: race.date,
            racecourse: race.racecourse,
            num: race.num,
            weather: scrap_str(
                "ul.trackState:nth-child(2) > li:nth-child(2) > dl:nth-child(1) > dd:nth-child(2)",
                &self,
            ),
            track_condition: scrap_str(
                "ul.trackState:nth-child(2) > li:nth-child(2) > dl:nth-child(1) > dd:nth-child(4)",
                &self,
            ),
            post_time: scrap_str(
                "ul.trackState:nth-child(2) > li:nth-child(2) > dl:nth-child(1) > dd:nth-child(6)",
                &self,
            ),
            name: track_main_state.clone(),
            class: track_main_state.clone(),
            horse_condition: scrap_str(".horseCondition > li:nth-child(1)", &self),
        }
    }
}

pub fn scrap_str(selectors: &str, doc: &Html) -> String {
    let weatcher_selector = Selector::parse(selectors).unwrap();
    let hoge = doc.select(&weatcher_selector).next().unwrap();
    let r = hoge.text().collect::<Vec<_>>()[0];
    r.nfkc().collect::<String>()
}
