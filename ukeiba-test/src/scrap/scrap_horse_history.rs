use super::fetch_and_scrap_all;
use crate::db::{make_conn, Horses};
use rusqlite::params;
use ukeiba_scraper::horse_history;

pub fn scrap() {
    let conn = make_conn().unwrap();

    // horse_bajikyo_idを取得するクエリ
    let query = "SELECT horse_nar_id FROM horses";

    // クエリを実行し、結果を取得
    let mut stmt = conn.prepare(query).unwrap();
    let rows = stmt.query_map([], |row| row.get(0)).unwrap();

    // horse_nar_ids<String>に格納
    let horse_nar_ids: Vec<i64> = rows.map(|row| row.unwrap()).collect();

    let pages = horse_nar_ids
        .iter()
        .map(|x| horse_history::Page {
            horse_nar_id: x.clone(),
        })
        .collect::<Vec<_>>();

    let data = fetch_and_scrap_all(pages);
    let mut horse_data: Vec<Horses> = Vec::new();
    for datum in data {
        horse_data.push(Horses {
            horse_nar_id: Some(datum.horse_nar_id),
            horse_name: Some(datum.horse_name),
            horse_status: Some(datum.horse_status),
            deregistration_date: datum.deregistration_date,
            ..Default::default()
        })
    }

    let mut conn = make_conn().unwrap();
    let tx = conn.transaction().unwrap();
    for horse_datum in horse_data {
        tx.execute(
            "INSERT INTO horses
            (horse_nar_id, horse_name, horse_status, deregistration_date)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(horse_nar_id) DO UPDATE SET
            horse_name = COALESCE(?2, horses.horse_name),
            horse_status = COALESCE(?3, horses.horse_status),
            deregistration_date = COALESCE(?4, horses.deregistration_date)",
            params![
                horse_datum.horse_nar_id,
                horse_datum.horse_name,
                horse_datum.horse_status,
                horse_datum.deregistration_date,
            ],
        )
        .unwrap();
    }
    tx.commit().unwrap();

    // horse_bajikyo_idsを利用する
}
