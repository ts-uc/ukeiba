use crate::common::*;
use crate::db::{
    writer::{write_to_db, DbWriter},
    Jockeys,
};
use crate::get::get_jockey_nar_id;
use ukeiba_common::scraper::jockey_profile;

pub fn scrap() {
    let jockey_all_ids = get_jockey_nar_id::get_all_from_nar();

    let jockey_profile_pages = jockey_all_ids
        .into_iter()
        .map(|jockey_nar_id| jockey_profile::Page {
            jockey_nar_id: jockey_nar_id.0,
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
