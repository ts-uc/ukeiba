//#![deny(warnings)]
use chrono::prelude::*;
use scraper::{Selector};

enum Racecourse {
    Obihiro,
}

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

fn make_race_id(race: Race) -> String {
    format!(
        "{}{:02}000000{:02}",
        race.date.format("%Y%m%d"),
        match race.racecourse {
            Racecourse::Obihiro => 3,
        },
        race.num
    )
}

fn scraping(race: Race) -> Result<(), Box<dyn std::error::Error>> {
    let race_id = make_race_id(race);
    let url: String = format!(
        "https://keiba.rakuten.co.jp/race_performance/list/RACEID/{}",
        race_id
    );

    eprintln!("Fetching {:?}...", url);

    let res = reqwest::blocking::get(url)?;

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    let body: String = res.text()?;
    let doc = scraper::Html::parse_document(&body);

    let weatcher_selector = Selector::parse("ul.trackState:nth-child(2) > li:nth-child(2) > dl:nth-child(1) > dd:nth-child(2)").unwrap();

    let hoge = doc.select(&weatcher_selector).next().unwrap();
    println!("{:?}", hoge.text().collect::<Vec<_>>()[0]);

    Ok(())
}
