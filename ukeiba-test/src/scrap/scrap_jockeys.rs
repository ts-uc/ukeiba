use super::*;
use crate::common::*;
use crate::db::{
    writer::{write_to_db, DbWriter},
    Jockeys,
};
use rayon::prelude::*;
use ukeiba_common::{
    common::HorseBelong,
    scraper::{jockey_profile, jockey_search},
};

pub fn scrap() {
    // 所属がばんえいか退厩の馬を全取得

    let pages: Vec<jockey_search::Page> = [jockey_search::Page {
        page_num: 1,
        belong: HorseBelong::Banei,
    }]
    .to_vec();

    let search_pages: Vec<jockey_search::Page> = fetch_and_scrap_all(pages)
        .par_iter()
        .map(|page| {
            let hits = page.hits;
            if hits == 0 {
                return Vec::new();
            }
            let page_count = (hits - 1) / 50 + 1;
            (1..=page_count)
                .map(|page_num| jockey_search::Page {
                    page_num: page_num,
                    belong: page.belong,
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>()
        .concat();

    let jockey_all_ids = fetch_and_scrap_all(search_pages)
        .into_iter()
        .flat_map(|data| data.jockey_ids)
        .collect::<Vec<_>>();

    let jockey_profile_pages = jockey_all_ids
        .iter()
        .map(|jockey_nar_id| jockey_profile::Page {
            jockey_nar_id: *jockey_nar_id,
        })
        .collect::<Vec<_>>();

    let jockey_profile_pages = fetch_and_scrap_all(jockey_profile_pages);

    //DBへ書き込むデータを作成

    let jockeys = jockey_profile_pages
        .into_iter()
        .map(|data| Jockeys {
            jockey_nar_id: data.jockey_nar_id,
            jockey_name: data.name,
            jockey_kana: data.kana,
            jockey_sex: data.sex,
            jockey_status: data.status,
            jockey_birthdate: data.birthdate,
            jockey_first_run: data.first_run,
            jockey_first_win: data.first_win,
        })
        .collect::<Vec<_>>();

    let db_writer = jockeys
        .into_iter()
        .map(|x| DbWriter::JockeysToJockeys(x))
        .collect::<Vec<_>>();

    write_to_db(&db_writer);
}
