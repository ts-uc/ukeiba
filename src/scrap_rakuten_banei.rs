pub mod racedata;

extern crate unicode_normalization;

use scraper::Html;
use crate::scrap_rakuten_banei::racedata::RaceDataBaneiTrait;

use super::Race;
use super::Racecourse;

enum PageType {
    Odds,
    Results,
}

impl Race {
    fn get_race_id(&self) -> String {
        format!(
            "{}{:02}000000{:02}",
            self.date.format("%Y%m%d"),
            match self.racecourse {
                Racecourse::Obihiro => 3,
                Racecourse::Monbetsu => todo!()
            },
            self.num
        )
    }

    fn get_url(&self, page_type: &PageType) -> String {
        match page_type {
            PageType::Odds => format!(
                "https://keiba.rakuten.co.jp/odds/tanfuku/RACEID/{}",
                self.get_race_id()
            ),
            PageType::Results => format!(
                "https://keiba.rakuten.co.jp/race_performance/list/RACEID/{}",
                self.get_race_id()
            ),
        }
    }

    fn fetch(&self, page_type: PageType) -> Html {
        eprintln!("Fetching {:?}...", self.get_url(&page_type));
        let res = reqwest::blocking::get(self.get_url(&page_type)).unwrap();
        eprintln!("Response: {:?} {}", res.version(), res.status());
        let res = res.text().unwrap().to_string();
        Html::parse_document(&res)
    }
}

pub fn scrap(race: Race) -> Result<(), Box<dyn std::error::Error>> {
    let body_odds = race.fetch(PageType::Odds);

    let racedata = body_odds.scrap_race_data(race);
    println!("{:?}", racedata);

    Ok(())
}
