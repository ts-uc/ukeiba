use super::*;
use crate::db::{make_conn, Jockeys};
use serde_rusqlite::to_params_named;
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
            name: data.name,
            kana: data.kana,
            sex: data.sex,
            status: data.status,
            birthdate: data.birthdate,
            first_run: data.first_run,
            first_win: data.first_win,
        })
        .collect::<Vec<_>>();

    let mut conn = make_conn().unwrap();
    let tx = conn.transaction().unwrap();
    for datum in jockeys {
        tx.execute(
            "
                INSERT INTO jockeys
                (jockey_nar_id, name, kana, sex, status,
                    birthdate, first_run, first_win)
                VALUES 
                (:jockey_nar_id, :name, :kana, :sex, :status,
                :birthdate, :first_run, :first_win)
                ON CONFLICT(jockey_nar_id) DO UPDATE SET
                name = COALESCE(jockeys.name, :name),
                kana = COALESCE(:kana, jockeys.kana),
                sex = COALESCE(:sex, jockeys.sex),
                status = COALESCE(:status, jockeys.status),
                birthdate = COALESCE(:birthdate, jockeys.birthdate),
                first_run = COALESCE(:first_run, jockeys.first_run),
                first_win = COALESCE(:first_win, jockeys.first_win)
            ",
            to_params_named(&datum).unwrap().to_slice().as_slice(),
        )
        .unwrap();
    }
    tx.commit().unwrap();
}
