extern crate ukeiba_scraper;
use anyhow::Result;
use chrono::NaiveDate;
use csv::Writer;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator};
use itertools::{iproduct, Itertools};
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
    let mut search_pages: Vec<horse_search::Page> = Vec::new();
    for (belong, year) in iproduct!(
        [
            horse_search::HorseBelong::Banei,
            horse_search::HorseBelong::Left
        ],
        (1969..=2021).rev()
    ) {
        println!("{:?} {:?}", belong, year);
        let hits = horse_search::Page {
            page_num: 1,
            horse_name: "".to_string(),
            horse_belong: belong,
            birth_year: year,
        }
        .fetch_scrap(Mode::NormalSave, Duration::from_secs(1))
        .unwrap_or_default()
        .hits_all;

        if hits == 0 {
            continue;
        } else if 0 < hits && hits <= 2000 {
            let pages = (hits - 1) / 50 + 1;
            let tmp_search_pages: Vec<horse_search::Page> = (1..=pages)
                .map(|page_num| horse_search::Page {
                    page_num: page_num,
                    horse_name: "".to_string(),
                    horse_belong: belong,
                    birth_year: year,
                })
                .collect();
            search_pages.extend(tmp_search_pages);
        } else {
            for kana in "アイウエオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモヤユヨラリルレロワヲンヴ".chars() {
                println!("{}", kana);
                let hits = horse_search::Page {
                    page_num: 1,
                    horse_name: kana.to_string(),
                    horse_belong: belong,
                    birth_year: year,
                }
                .fetch_scrap(Mode::NormalSave, Duration::from_secs(1))
                .unwrap_or_default()
                .hits_all;
                if hits == 0 {
                    continue;
                } else if 0 < hits {
                    let pages = (hits - 1) / 50 + 1;
                    let tmp_search_pages: Vec<horse_search::Page> = (1..=pages)
                        .map(|page_num| horse_search::Page {
                            page_num: page_num,
                            horse_name: kana.to_string(),
                            horse_belong: belong,
                            birth_year: year,
                        })
                        .collect();
                    search_pages.extend(tmp_search_pages);
                }
            }
        }
    }

    for page in search_pages.clone() {
        println!("{:?}", page);
        match page.fetch(Duration::from_secs(1)) {
            Ok(_) => (),
            Err(_) => continue,
        };
    }

    let horses: Vec<Vec<i64>> = search_pages
        .par_iter()
        .progress_count(search_pages.len() as u64)
        .map(|page| page.scrap())
        .filter_map(Result::ok)
        .map(|data| data.data.iter().map(|x| x.horse_nar_id).collect())
        .collect();

    let horses: Vec<i64> = horses.into_iter().flat_map(|x| x).collect();

    let pages: Vec<horse_profile::Page> = horses
        .into_iter()
        .map(|horse_nar_id| horse_profile::Page {
            horse_nar_id: horse_nar_id,
        })
        .collect();

    for page in pages.clone() {
        match page.fetch(Duration::from_secs(1)) {
            Ok(_) => (),
            Err(_) => continue,
        };
    }

    let horses: Vec<HorseData> = pages
        .par_iter()
        .progress_count(pages.len() as u64)
        .map(|page| page.scrap())
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
