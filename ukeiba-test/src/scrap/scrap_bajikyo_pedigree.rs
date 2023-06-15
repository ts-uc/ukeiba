use super::fetch_and_scrap_all;
use crate::db::{make_conn, Horses};
use rusqlite::params;
use ukeiba_scraper::bajikyo_pedigree;

pub fn scrap() {
    let conn = make_conn().unwrap();

    // horse_bajikyo_idを取得するクエリ
    let query = "SELECT horse_bajikyo_id FROM horses";

    // クエリを実行し、結果を取得
    let mut stmt = conn.prepare(query).unwrap();
    let rows = stmt.query_map([], |row| row.get(0)).unwrap();

    // horse_bajikyo_idの値をVec<String>に格納
    let horse_bajikyo_ids: Vec<String> = rows.map(|row| row.unwrap()).collect();

    let bajikyo_pedigree_pages = horse_bajikyo_ids
        .iter()
        .map(|x| bajikyo_pedigree::Page {
            horse_bajikyo_id: x.clone(),
        })
        .collect::<Vec<_>>();

    let bajikyo_pedigree_data = fetch_and_scrap_all(bajikyo_pedigree_pages);
    let horse_data = bajikyo_pedigree_data
        .into_iter()
        .map(|data| Horses {
            horse_bajikyo_id: Some(data.horse_bajikyo_id),
            sire_bajikyo_id: data.sire_bajikyo_id,
            dam_bajikyo_id: data.dam_bajikyo_id,
            bms_bajikyo_id: data.bms_bajikyo_id,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    let mut conn = make_conn().unwrap();
    let tx = conn.transaction().unwrap();
    for horse_datum in horse_data {
        tx.execute(
            "INSERT INTO horses
            (horse_bajikyo_id, sire_bajikyo_id, dam_bajikyo_id, bms_bajikyo_id)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(horse_bajikyo_id) DO UPDATE SET
            sire_bajikyo_id = COALESCE(?2, horses.sire_bajikyo_id),
            dam_bajikyo_id = COALESCE(?3, horses.dam_bajikyo_id),
            bms_bajikyo_id = COALESCE(?4, horses.bms_bajikyo_id)",
            params![
                horse_datum.horse_bajikyo_id,
                horse_datum.sire_bajikyo_id,
                horse_datum.dam_bajikyo_id,
                horse_datum.bms_bajikyo_id,
            ],
        )
        .unwrap();
    }
    tx.commit().unwrap();

    // horse_bajikyo_idsを利用する
}
