extern crate unicode_normalization;

use super::Race;
use super::Racecourse;

use unicode_normalization::UnicodeNormalization;
use chrono::prelude::*;
use scraper::Html;
use scraper::Selector;

impl Race {
    fn race_id(&self) -> String {
        format!(
            "{}{:02}000000{:02}",
            self.date.format("%Y%m%d"),
            match self.racecourse {
                Racecourse::Obihiro => 3,
            },
            self.num
        )
    }
    fn url(&self) -> String {
        format!(
            "https://keiba.rakuten.co.jp/race_performance/list/RACEID/{}",
            self.race_id()
        )
    }
}

#[derive(Debug)]
struct RaceDataBanei {
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

fn scrap_racedata_banei(doc: &Html, race: Race) -> RaceDataBanei {
    RaceDataBanei {
        date: race.date,
        racecourse: race.racecourse,
        num: race.num,
        weather: scrap_str(
            "ul.trackState:nth-child(2) > li:nth-child(2) > dl:nth-child(1) > dd:nth-child(2)",
            &doc,
        ),
        track_condition: scrap_str("ul.trackState:nth-child(2) > li:nth-child(2) > dl:nth-child(1) > dd:nth-child(4)", &doc),
        post_time: scrap_str("ul.trackState:nth-child(2) > li:nth-child(2) > dl:nth-child(1) > dd:nth-child(6)", &doc),
        name: scrap_str(".raceNote > h2:nth-child(3)", &doc),
        class: scrap_str(".raceNote > h2:nth-child(3)", &doc),
        class_sub: scrap_str(".raceNote > h2:nth-child(3)", &doc),
        horse_condition: scrap_str(".horseCondition > li:nth-child(1)", &doc),
    }
}

fn scrap_str(selectors: &str, doc: &Html) -> String {
    let weatcher_selector = Selector::parse(selectors).unwrap();
    let hoge = doc.select(&weatcher_selector).next().unwrap();
    let r = hoge.text().collect::<Vec<_>>()[0];
    r.nfkc().collect::<String>()
}

pub fn scrap(race: Race) -> Result<(), Box<dyn std::error::Error>> {
    let url = race.url();
    eprintln!("Fetching {:?}...", url);
    let res = reqwest::blocking::get(url)?;
    eprintln!("Response: {:?} {}", res.version(), res.status());

    let body: String = res.text()?;
    let doc = scraper::Html::parse_document(&body);

    let racedata = scrap_racedata_banei(&doc, race);
    println!("{:?}", racedata);

    Ok(())
}
