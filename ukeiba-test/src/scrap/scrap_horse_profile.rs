use crate::common::*;
use crate::db::writer::{write_to_db, DbWriter};
use crate::db::Horses;
use crate::get::get_horse_nar_id;
use std::collections::HashMap;
use ukeiba_common::scraper::{bajikyo_auto_search, horse_profile};

pub fn scrap() {
    let horse_all_ids = get_horse_nar_id::get_all_from_nar();

    // 取得した全馬のIDリストから、サラブレッド種・サラブレッド系種・アングロアラブ種を除外した馬情報リストを作成

    let horse_profile_pages = horse_all_ids
        .into_iter()
        .map(|horse_nar_id| horse_profile::Page {
            horse_nar_id: horse_nar_id.0,
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
