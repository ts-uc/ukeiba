extern crate ukeiba_scraper;
use anyhow::Result;
use chrono::NaiveDate;
use csv::Writer;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use itertools::iproduct;
use rayon::prelude::*;
use serde::Serialize;
use std::time::Duration;
use ukeiba_scraper::{horse_history, horse_profile, horse_search, WebPageTrait};
pub mod db;

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
    let pages: Vec<horse_search::Page> = iproduct!(
        [
            horse_search::HorseBelong::Banei,
            horse_search::HorseBelong::Left
        ],
        (1969..=2021).rev()
    )
    .map(|(belong, year)| horse_search::Page {
        page_num: 1,
        horse_name: "".to_string(),
        horse_belong: belong,
        birth_year: year,
    })
    .collect();

    fetch_all(&pages);

    let search_pages: Vec<horse_search::Page> = pages
        .par_iter()
        .progress_count(pages.len() as u64)
        .map(|page| {
            let hits = page.scrap().unwrap_or_default().hits_all;
            match hits {
                0 => Vec::new(),
                0..=2000 => [page.clone()].to_vec(),
                _ => "アイウエオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモヤユヨラリルレロワヲンヴ"
                    .chars()
                    .map(|kana| horse_search::Page {
                        page_num: 1,
                        horse_name: kana.to_string(),
                        horse_belong: page.horse_belong,
                        birth_year: page.birth_year,
                    })
                    .collect(),
            }
        })
        .collect::<Vec<Vec<_>>>()
        .concat();

    fetch_all(&search_pages);

    let search_pages: Vec<horse_search::Page> = search_pages
        .par_iter()
        .progress_count(search_pages.len() as u64)
        .map(|page| {
            let hits = page.scrap().unwrap_or_default().hits_all;
            if hits == 0 {
                return Vec::new();
            }
            let page_count = (hits - 1) / 50 + 1;
            (1..=page_count)
                .map(|page_num| horse_search::Page {
                    page_num: page_num,
                    horse_name: page.horse_name.clone(),
                    horse_belong: page.horse_belong,
                    birth_year: page.birth_year,
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>()
        .concat();

    fetch_all(&search_pages);

    let horses: Vec<i64> = search_pages
        .par_iter()
        .progress_count(search_pages.len() as u64)
        .map(|page| page.scrap())
        .filter_map(Result::ok)
        .map(|data| data.data.iter().map(|x| x.horse_nar_id).collect())
        .collect::<Vec<Vec<_>>>()
        .concat();

    let pages: Vec<horse_profile::Page> = horses
        .iter()
        .map(|horse_nar_id| horse_profile::Page {
            horse_nar_id: *horse_nar_id,
        })
        .collect();

    fetch_all(&pages);

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

    let horse_history_pages: Vec<horse_history::Page> = horses
        .iter()
        .map(|horse| horse_history::Page {
            horse_nar_id: horse.horse_nar_id,
        })
        .collect();

    fetch_all(&horse_history_pages);

    let horse_history_data_row: Vec<horse_history::DataRow> = horse_history_pages
        .par_iter()
        .progress_count(horse_history_pages.len() as u64)
        .map(|page| page.scrap())
        .filter_map(Result::ok)
        .map(|data| data.data)
        .collect::<Vec<Vec<_>>>()
        .concat();

    write_csv("races.csv", &horse_history_data_row).unwrap();
}

//3659958

fn fetch_all<T: WebPageTrait>(pages: &[T]) {
    pages
        .iter()
        .progress()
        .filter_map(|page| page.fetch(Duration::from_secs(1)).ok())
        .for_each(drop);
}

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

// fn to_bajikyo_id(nar_id: i64) -> String {
//     let chars: Vec<char> = nar_id.to_string().chars().collect();
//     let shuffled: i64 = format!(
//         "{}{}{}{}{}{}{}{}{}{}",
//         chars[5],
//         chars[1],
//         chars[10],
//         chars[9],
//         chars[2],
//         chars[0],
//         chars[4],
//         chars[8],
//         chars[3],
//         chars[7]
//     )
//     .parse()
//     .unwrap();
//     let mut num_chars: Vec<char> = (shuffled - 2046971875).to_string().chars().rev().collect();

//     if num_chars.len() >= 5 {
//         if num_chars[4] == '5' {
//             num_chars[4] = ' ';
//         } else if num_chars[4] == '4' {
//             num_chars[4] = 'H';
//         }
//     }

//     num_chars.iter().rev().collect()
// }
