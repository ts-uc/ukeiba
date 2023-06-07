extern crate ukeiba_scraper;
use anyhow::Result;
use chrono::NaiveDate;
use csv::Writer;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use itertools::iproduct;
use rayon::prelude::*;
use serde::Serialize;
use std::time::Duration;
use ukeiba_scraper::{
    bajikyo_auto_search, bajikyo_pedigree, bajikyo_profile, horse_history, horse_profile,
    horse_search, WebPageTrait,
};
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
    // 所属がばんえいか退厩の馬を全取得

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

    let search_pages: Vec<horse_search::Page> = fetch_and_scrap_all(pages)
        .par_iter()
        .map(|page| {
            let hits = page.hits_all;
            match hits {
                0 => Vec::new(),
                0..=2000 => [horse_search::Page {
                    page_num: 1,
                    horse_name: "".to_string(),
                    horse_belong: page.horse_belong,
                    birth_year: page.birth_year,
                }]
                .to_vec(),
                _ => "アイウエオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモヤユヨラリルレロワヲンヴ"
                    .chars()
                    .map(|kana| horse_search::Page {
                        page_num: 1,
                        horse_name: kana.to_string(),
                        horse_belong: page.horse_belong,
                        birth_year: page.birth_year,
                    })
                    .collect::<Vec<_>>(),
            }
        })
        .collect::<Vec<Vec<_>>>()
        .concat();

    let search_pages: Vec<horse_search::Page> = fetch_and_scrap_all(search_pages)
        .par_iter()
        .map(|page| {
            let hits = page.hits_all;
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

    let horse_all_ids = fetch_and_scrap_all(search_pages)
        .into_iter()
        .flat_map(|data| data.data.iter().map(|x| x.horse_nar_id).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // 取得した全馬のIDリストから、サラブレッド種・サラブレッド系種・アングロアラブ種を除外した馬情報リストを作成

    let horse_profile_pages = horse_all_ids
        .iter()
        .map(|horse_nar_id| horse_profile::Page {
            horse_nar_id: *horse_nar_id,
        })
        .collect::<Vec<_>>();

    let horse_profile_data = fetch_and_scrap_all(horse_profile_pages)
        .into_iter()
        .filter(|data| match data.horse_type.as_deref() {
            Some("(アア)") | Some("(サラ系)") | None => false,
            _ => true,
        })
        .collect::<Vec<_>>();

    // 馬情報リストの情報をベースに馬事協会IDを取得

    let horse_data = horse_profile_data
        .iter()
        .map(|data| get_horse_profile(data.clone()))
        .collect::<Vec<_>>();

    write_csv("horses.csv", &horse_data).unwrap();

    let bajikyo_searched_data = fetch_and_scrap_all(horse_data);

    let bajikyo_ids = bajikyo_searched_data
        .iter()
        .filter_map(|x| x.horse_bajikyo_id.clone())
        .collect::<Vec<_>>();

    // 馬事協会のサイトから父馬ID・母馬ID・母父馬IDを取得

    let bajikyo_pedigree_pages = bajikyo_ids
        .iter()
        .map(|x| bajikyo_pedigree::Page {
            horse_bajikyo_id: x.clone(),
        })
        .collect::<Vec<_>>();

    let bajikyo_pedigree_data = fetch_and_scrap_all(bajikyo_pedigree_pages);
    write_csv("bajikyo_pedigree.csv", &bajikyo_pedigree_data).unwrap();

    // 馬事協会のサイトから馬情報を取得

    let bajikyo_profile_pages = bajikyo_ids
        .iter()
        .map(|x| bajikyo_profile::Page {
            horse_bajikyo_id: x.clone(),
        })
        .collect::<Vec<_>>();

    let bajikyo_profile_data = fetch_and_scrap_all(bajikyo_profile_pages);
    write_csv("bajikyo_profile.csv", &bajikyo_profile_data).unwrap();

    write_csv("bajikyo_data.csv", &bajikyo_searched_data).unwrap();

    // let horse_history_pages: Vec<horse_history::Page> = horse_data
    //     .iter()
    //     .map(|horse| horse_history::Page {
    //         horse_nar_id: horse.horse_nar_id,
    //     })
    //     .collect();

    // let horse_history_data_row: Vec<horse_history::DataRow> =
    //     fetch_and_scrap_all(horse_history_pages)
    //         .into_iter()
    //         .flat_map(|data| data.data)
    //         .collect::<Vec<_>>();

    // write_csv("races.csv", &horse_history_data_row).unwrap();
}

//3659958

fn fetch_and_scrap_all<T>(pages: Vec<T>) -> Vec<T::Data>
where
    T::Data: Send,
    T: WebPageTrait + Sync,
{
    fetch_all(&pages);
    scrap_all(pages)
}

fn fetch_all<T: WebPageTrait>(pages: &[T]) {
    pages
        .iter()
        .progress()
        .filter_map(|page| page.fetch(Duration::from_secs(2)).ok())
        .for_each(drop);
}

fn scrap_all<T>(pages: Vec<T>) -> Vec<T::Data>
where
    T::Data: Send,
    T: WebPageTrait + Sync,
{
    pages
        .par_iter()
        .progress_count(pages.len() as u64)
        .map(|page| page.scrap())
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

fn get_horse_profile(data: horse_profile::Data) -> bajikyo_auto_search::OriginalData {
    bajikyo_auto_search::OriginalData {
        horse_nar_id: data.horse_nar_id,
        horse_name: data.horse_name,
        birthdate: data.birthdate.unwrap_or_default(),
        sire_name: data.sire_name.unwrap_or_default(),
        dam_name: data.dam_name.unwrap_or_default(),
    }
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
