//#![deny(warnings)]
extern crate unicode_normalization;

use unicode_normalization::UnicodeNormalization;
use chrono::prelude::*;
use scraper::Html;
use scraper::Selector;

#[derive(Debug)]
enum Racecourse {
    Obihiro,
}

#[derive(Debug)]
struct Race {
    date: Date<Local>,
    racecourse: Racecourse,
    num: i32,
}

fn main() {
    env_logger::init();
    let puri_puri_pudding = Race {
        date: Local.ymd(2022, 8, 20),
        racecourse: Racecourse::Obihiro,
        num: 7,
    };

    let _ = scraping(puri_puri_pudding);
}

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
    race: Race,
    distance: String,
    weather: String,
    track_condition: String,
    name: String,
    class: String,
    class_sub: String,
    horse_condition: String,
    prize1: String,
    prize2: String,
    prize3: String,
    prize4: String,
    prize5: String,
}

fn scrap_racedata_banei(doc: &Html, race: Race) -> RaceDataBanei {
    RaceDataBanei {
        race: race,
        distance: scrap_str(".distance", &doc),
        weather: scrap_str(
            "ul.trackState:nth-child(2) > li:nth-child(2) > dl:nth-child(1) > dd:nth-child(2)",
            &doc,
        ),
        track_condition: scrap_str("ul.trackState:nth-child(2) > li:nth-child(2) > dl:nth-child(1) > dd:nth-child(4)", &doc),
        name: scrap_str(".raceNote > h2:nth-child(3)", &doc),
        class: scrap_str(".raceNote > h2:nth-child(3)", &doc),
        class_sub: scrap_str(".raceNote > h2:nth-child(3)", &doc),
        horse_condition: scrap_str(".horseCondition > li:nth-child(1)", &doc),
        prize1: scrap_str(".prizeMoney > dd:nth-child(2) > ol:nth-child(1) > li:nth-child(1)", &doc),
        prize2: scrap_str(".prizeMoney > dd:nth-child(2) > ol:nth-child(1) > li:nth-child(2)", &doc),
        prize3: scrap_str(".prizeMoney > dd:nth-child(2) > ol:nth-child(1) > li:nth-child(3)", &doc),
        prize4: scrap_str(".prizeMoney > dd:nth-child(2) > ol:nth-child(1) > li:nth-child(4)", &doc),
        prize5: scrap_str(".prizeMoney > dd:nth-child(2) > ol:nth-child(1) > li:nth-child(5)", &doc),
    }
}

fn scrap_str(selectors: &str, doc: &Html) -> String {
    let weatcher_selector = Selector::parse(selectors).unwrap();
    let hoge = doc.select(&weatcher_selector).next().unwrap();
    let r = hoge.text().collect::<Vec<_>>()[0];
    r.nfkc().collect::<String>()
}

fn scraping(race: Race) -> Result<(), Box<dyn std::error::Error>> {
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
