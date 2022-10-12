pub mod racedata;

extern crate unicode_normalization;

use super::Race;
use super::Racecourse;

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
    fn url_result(&self) -> String {
        format!(
            "https://keiba.rakuten.co.jp/race_performance/list/RACEID/{}",
            self.race_id()
        )
    }
    fn url_odds(&self) -> String {
        format!(
            "https://keiba.rakuten.co.jp/odds/tanfuku/RACEID/{}",
            self.race_id()
        )
    }
    fn fetch_result(&self) -> String {
        eprintln!("Fetching {:?}...", self.url_result());
        let res = reqwest::blocking::get(self.url_result()).unwrap();
        eprintln!("Response: {:?} {}", res.version(), res.status());
        res.text().unwrap().to_string()
    }
    fn fetch_odds(&self) -> String {
        eprintln!("Fetching {:?}...", self.url_odds());
        let res = reqwest::blocking::get(self.url_odds()).unwrap();
        eprintln!("Response: {:?} {}", res.version(), res.status());
        res.text().unwrap().to_string()
    }
}

pub fn scrap(race: Race) -> Result<(), Box<dyn std::error::Error>> {
    let body: String = race.fetch_result();

    let racedata = racedata::RaceDataBanei::scrap(&body, race);
    println!("{:?}", racedata);

    Ok(())
}
