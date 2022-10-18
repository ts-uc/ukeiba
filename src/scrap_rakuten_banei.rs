pub mod racedata;

extern crate unicode_normalization;

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

    fn fetch(&self, page_type: PageType) -> String {
        eprintln!("Fetching {:?}...", self.get_url(&page_type));
        let res = reqwest::blocking::get(self.get_url(&page_type)).unwrap();
        eprintln!("Response: {:?} {}", res.version(), res.status());
        res.text().unwrap().to_string()
    }
}

pub fn scrap(race: Race) -> Result<(), Box<dyn std::error::Error>> {
    let body_odds: String = race.fetch(PageType::Odds);

    let racedata = racedata::RaceDataBanei::scrap(&body_odds, race);
    println!("{:?}", racedata);

    Ok(())
}
