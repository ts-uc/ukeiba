extern crate ukeiba_scraper;
use anyhow::Result;
use ukeiba_scraper::{
    Mode, WebPageTrait,
};

fn main() {
    // let a = jockey_profile::Page {
    //     jockey_nar_id: 38077,
    // }
    // .get(Mode::NormalSave, std::time::Duration::from_secs(1))
    // .unwrap()
    // .scrap();
    // println!("{:#?}", a);

    for year in (1969..=2021).rev() {
        println!("{}", year);
        let page_data = ukeiba_scraper::horse_search::Page {
            page_num: 1,
            horse_name: "".to_string(),
            horse_belong: ukeiba_scraper::horse_search::HorseBelong::Banei,
            birth_year: year,
        }
        .get(Mode::NormalSave, std::time::Duration::from_secs(1))
        .unwrap()
        .scrap()
        .unwrap();
        if page_data.hits > 0 {
            let pages = (page_data.hits - 1) / 50 + 1;
            for page in 1..=pages {
                let page_data = ukeiba_scraper::horse_search::Page {
                    page_num: page,
                    horse_name: "".to_string(),
                    horse_belong: ukeiba_scraper::horse_search::HorseBelong::Banei,
                    birth_year: year,
                }
                .get(Mode::NormalSave, std::time::Duration::from_secs(1))
                .unwrap()
                .scrap()
                .unwrap();
                let horses = page_data.data.iter().map(|x| x.horse_nar_id);
                for horse_nar_id in horses {
                    match get_horse_profile(horse_nar_id){
                        Ok(_) => (),
                        Err(_) => continue,
                }
            }
            }
        }
    }

    for year in (1969..=2021).rev() {
        for kana in "アイウエオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモヤユヨラリルレロワヲンヴ".chars() {
            println!("{}", kana);
            println!("{}", year);
            let page_data = ukeiba_scraper::horse_search::Page {
                page_num: 1,
                horse_name: kana.to_string(),
                horse_belong: ukeiba_scraper::horse_search::HorseBelong::Left,
                birth_year: year,
            }
            .get(Mode::NormalSave, std::time::Duration::from_secs(1))
            .unwrap()
            .scrap()
            .unwrap();
            if page_data.hits > 0 {
                let pages = (page_data.hits - 1) / 50 + 1;
                for page in 1..=pages {
                    let page_data = ukeiba_scraper::horse_search::Page {
                        page_num: page,
                        horse_name: kana.to_string(),
                        horse_belong: ukeiba_scraper::horse_search::HorseBelong::Left,
                        birth_year: year,
                    }
                    .get(Mode::NormalSave, std::time::Duration::from_secs(1))
                    .unwrap()
                    .scrap()
                    .unwrap();
                    let horses = page_data.data.iter().map(|x| x.horse_nar_id);
                    for horse_nar_id in horses {
                        match get_horse_profile(horse_nar_id){
                            Ok(_) => (),
                            Err(_) => continue,
                        }
                    }
                }
            }
    
       }
   
    }
}

//3659958

fn get_horse_profile(horse_nar_id: i64) -> Result<()> {
    let data = ukeiba_scraper::horse_profile::Page {
        horse_nar_id: horse_nar_id,
    }
    .get(Mode::NormalSave, std::time::Duration::from_secs(1))?.scrap()?;
    println!("{:#?}", data);
    Ok(())
}

