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
    class_sub: String,
    horse_condition: String,
}

impl RaceDataBanei {
    pub fn scrap(body: &str, race: Race) -> RaceDataBanei {
        let doc = scraper::Html::parse_document(&body);

        RaceDataBanei {
            date: race.date,
            racecourse: race.racecourse,
            num: race.num,
            weather: scrap_str(
                "ul.trackState:nth-child(2) > li:nth-child(2) > dl:nth-child(1) > dd:nth-child(2)",
                &doc,
            ),
            track_condition: scrap_str(
                "ul.trackState:nth-child(2) > li:nth-child(2) > dl:nth-child(1) > dd:nth-child(4)",
                &doc,
            ),
            post_time: scrap_str(
                "ul.trackState:nth-child(2) > li:nth-child(2) > dl:nth-child(1) > dd:nth-child(6)",
                &doc,
            ),
            name: scrap_str(".raceNote > h2:nth-child(3)", &doc),
            class: scrap_str(".raceNote > h2:nth-child(3)", &doc),
            class_sub: scrap_str(".raceNote > h2:nth-child(3)", &doc),
            horse_condition: scrap_str(".horseCondition > li:nth-child(1)", &doc),
        }
    }
}

pub fn scrap_str(selectors: &str, doc: &Html) -> String {
    let weatcher_selector = Selector::parse(selectors).unwrap();
    let hoge = doc.select(&weatcher_selector).next().unwrap();
    let r = hoge.text().collect::<Vec<_>>()[0];
    r.nfkc().collect::<String>()
}
