use super::*;
use crate::db::writer::{write_to_db, DbWriter};
use itertools::iproduct;
use std::collections::HashMap;
use ukeiba_common::{
    common::HorseBelong,
    scraper::{bajikyo_auto_search, horse_profile, horse_search},
};

pub fn scrap() {
    // 所属がばんえいか退厩の馬を全取得

    let pages: Vec<horse_search::Page> =
        iproduct!([HorseBelong::Banei, HorseBelong::Left], (1976..=2021).rev())
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
        .flat_map(|data| data.horse_nar_ids)
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

    let bajikyo_searched_data = fetch_and_scrap_all(horse_data);

    let bajikyo_auto_search_dict = bajikyo_searched_data
        .iter()
        .filter_map(|data| Some((data.horse_nar_id.clone(), data.horse_bajikyo_id.clone()?)))
        .collect::<HashMap<_, _>>();

    // DBへ書き込むデータを作成

    let horse_data = horse_profile_data
        .iter()
        .map(|data| Horses {
            horse_nar_id: Some(data.horse_nar_id),
            horse_bajikyo_id: bajikyo_auto_search_dict.get(&data.horse_nar_id).cloned(),
            ..Default::default()
        })
        .collect::<Vec<_>>();

    let db_writer = horse_data
        .into_iter()
        .map(|x| DbWriter::HorseProfileToHorses(x))
        .collect::<Vec<_>>();

    write_to_db(&db_writer);
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
