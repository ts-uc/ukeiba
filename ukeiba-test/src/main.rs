extern crate ukeiba_scraper;
use anyhow::Result;
use chrono::NaiveDate;
use csv::Writer;
use rayon::prelude::*;
use serde::Serialize;
use std::time::Duration;
use ukeiba_scraper::{horse_profile, horse_search, Mode, WebPageTrait};

#[derive(Debug, Clone, Serialize, Default)]
struct HorseData {
    pub horse_nar_id: i64,
    pub horse_name: String,
    pub birthdate: Option<NaiveDate>,
    pub sire_name: Option<String>,
    pub dam_name: Option<String>,
}

fn main() {
    sub()
}

fn sub() {
    let mut horses = Vec::new();
    for year in (1969..=2021).rev() {
        println!("{}", year);
        let page_data = horse_search::Page {
            page_num: 1,
            horse_name: "".to_string(),
            horse_belong: horse_search::HorseBelong::Banei,
            birth_year: year,
        }
        .fetch_scrap(Mode::NormalSave, Duration::from_secs(1))
        .unwrap();
        println!("{} {}", page_data.hits, year);
        if page_data.hits > 0 {
            let pages = (page_data.hits - 1) / 50 + 1;
            for page in 1..=pages {
                let page_data = horse_search::Page {
                    page_num: page,
                    horse_name: "".to_string(),
                    horse_belong: horse_search::HorseBelong::Banei,
                    birth_year: year,
                }
                .fetch_scrap(Mode::NormalSave, Duration::from_secs(1))
                .unwrap();
                horses.extend(page_data.data.iter().map(|x| x.horse_nar_id));
            }
        }
    }

    for year in (1969..=2021).rev() {
        for kana in "アイウエオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモヤユヨラリルレロワヲンヴ".chars() {
            let page_data = horse_search::Page {
                page_num: 1,
                horse_name: kana.to_string(),
                horse_belong: horse_search::HorseBelong::Left,
                birth_year: year,
            }
            .fetch_scrap(Mode::NormalSave, Duration::from_secs(1))
            .unwrap();
            println!("{} {} {}",page_data.hits, kana, year);
            if page_data.hits > 0 {
                let pages = (page_data.hits - 1) / 50 + 1;
                for page in 1..=pages {
                    let page_data = horse_search::Page {
                        page_num: page,
                        horse_name: kana.to_string(),
                        horse_belong: horse_search::HorseBelong::Left,
                        birth_year: year,
                    }
                    .fetch_scrap(Mode::NormalSave, Duration::from_secs(1))
                    .unwrap();
                    horses.extend(page_data.data.iter().map(|x| x.horse_nar_id));
                }
            }
        }
    }
    let horses: Vec<HorseData> = horses
        .par_iter()
        .map(|horse_nar_id| {
            horse_profile::Page {
                horse_nar_id: *horse_nar_id,
            }
            .fetch_scrap(Mode::NormalSave, Duration::from_secs(1))
        })
        .filter_map(Result::ok)
        .filter(|data| match data.horse_type.as_deref() {
            Some("(アア)") | Some("(サラ系)") | None => false,
            _ => true,
        })
        .map(|data| get_horse_profile(data).unwrap_or_default())
        .collect();

    write_csv("horses.csv", &horses).unwrap();
}

//3659958

fn get_horse_profile(data: horse_profile::Data) -> Option<HorseData> {
    Some(HorseData {
        horse_nar_id: data.horse_nar_id,
        horse_name: data.horse_name,
        birthdate: data.birthdate,
        sire_name: data.sire_name,
        dam_name: data.dam_name,
    })
}

fn write_csv<T>(filename: &str, data: &[T]) -> Result<()>
where
    T: Serialize,
{
    let mut writer = Writer::from_path(filename)?;

    for record in data {
        writer.serialize(record)?;
    }

    writer.flush()?;
    Ok(())
}

fn to_bajikyo_id(nar_id: i64) -> String {
    let chars: Vec<char> = nar_id.to_string().chars().collect();
    let shuffled: i64 = format!(
        "{}{}{}{}{}{}{}{}{}{}",
        chars[5],
        chars[1],
        chars[10],
        chars[9],
        chars[2],
        chars[0],
        chars[4],
        chars[8],
        chars[3],
        chars[7]
    )
    .parse()
    .unwrap();
    let mut num_chars: Vec<char> = (shuffled - 2046971875).to_string().chars().rev().collect();

    if num_chars.len() >= 5 {
        if num_chars[4] == '5' {
            num_chars[4] = ' ';
        } else if num_chars[4] == '4' {
            num_chars[4] = 'H';
        }
    }

    num_chars.into_iter().rev().collect()
}
