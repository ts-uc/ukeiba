extern crate ukeiba_scraper;
use ukeiba_scraper::{
    horse_history, horse_profile,
    horse_search::{self, HorseBelong},
    Mode, WebPageTrait,
};

fn main() {
    let a = horse_history::Page {
        horse_nar_id: 20142507675,
    }
    .get(Mode::NormalSave, std::time::Duration::from_secs(1))
    .unwrap()
    .scrap();
    println!("{:#?}", a);
}

//3659958
