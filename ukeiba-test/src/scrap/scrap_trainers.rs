use super::*;
use crate::db::{make_conn, Trainers};
use rusqlite::Transaction;
use serde_rusqlite::to_params_named;
use ukeiba_common::{
    common::HorseBelong,
    scraper::{trainer_profile, trainer_search},
};

pub fn scrap() {
    // 所属がばんえいか退厩の馬を全取得

    let pages: Vec<trainer_search::Page> = [trainer_search::Page {
        page_num: 1,
        belong: HorseBelong::Banei,
    }]
    .to_vec();

    let search_pages: Vec<trainer_search::Page> = fetch_and_scrap_all(pages)
        .par_iter()
        .map(|page| {
            let hits = page.hits;
            if hits == 0 {
                return Vec::new();
            }
            let page_count = (hits - 1) / 50 + 1;
            (1..=page_count)
                .map(|page_num| trainer_search::Page {
                    page_num: page_num,
                    belong: page.belong,
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>()
        .concat();

    let trainer_all_ids = fetch_and_scrap_all(search_pages)
        .into_iter()
        .flat_map(|data| data.trainer_ids)
        .collect::<Vec<_>>();

    let trainer_profile_pages = trainer_all_ids
        .iter()
        .map(|trainer_nar_id| trainer_profile::Page {
            trainer_nar_id: *trainer_nar_id,
        })
        .collect::<Vec<_>>();

    let trainer_profile_pages = fetch_and_scrap_all(trainer_profile_pages);

    //DBへ書き込むデータを作成

    let trainers = trainer_profile_pages
        .into_iter()
        .map(|data| Trainers {
            trainer_nar_id: data.trainer_nar_id,
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
    for datum in trainers {
        trainers_to_trainers(&tx, &datum)
    }
    tx.commit().unwrap();
}

fn trainers_to_trainers(tx: &Transaction, datum: &Trainers) {
    tx.execute(
        "
            INSERT INTO trainers
            (trainer_nar_id, name, kana, sex, status,
                birthdate, first_run, first_win)
            VALUES 
            (:trainer_nar_id, :name, :kana, :sex, :status,
            :birthdate, :first_run, :first_win)
            ON CONFLICT(trainer_nar_id) DO UPDATE SET
            name = COALESCE(trainers.name, :name),
            kana = COALESCE(:kana, trainers.kana),
            sex = COALESCE(:sex, trainers.sex),
            status = COALESCE(:status, trainers.status),
            birthdate = COALESCE(:birthdate, trainers.birthdate),
            first_run = COALESCE(:first_run, trainers.first_run),
            first_win = COALESCE(:first_win, trainers.first_win)
        ",
        to_params_named(&datum).unwrap().to_slice().as_slice(),
    )
    .unwrap();
}
