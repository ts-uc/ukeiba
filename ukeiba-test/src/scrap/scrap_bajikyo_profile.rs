use super::fetch_and_scrap_all;
use crate::db::{
    make_conn,
    writer::{write_to_db, DbWriter},
    Horses,
};
use ukeiba_common::scraper::bajikyo_profile;

pub fn scrap() {
    let conn = make_conn().unwrap();

    // horse_bajikyo_idを取得するクエリ
    let query = "SELECT horse_bajikyo_id FROM horses";

    // クエリを実行し、結果を取得
    let mut stmt = conn.prepare(query).unwrap();
    let rows = stmt.query_map([], |row| row.get(0)).unwrap();

    // horse_bajikyo_idの値をVec<String>に格納
    let horse_bajikyo_ids: Vec<String> = rows.map(|row| row.unwrap()).collect();

    let bajikyo_profile_pages = horse_bajikyo_ids
        .iter()
        .map(|x| bajikyo_profile::Page {
            horse_bajikyo_id: x.clone(),
        })
        .collect::<Vec<_>>();

    let bajikyo_profile_data = fetch_and_scrap_all(bajikyo_profile_pages);

    let horse_data = bajikyo_profile_data
        .into_iter()
        .map(|data| Horses {
            horse_bajikyo_id: Some(data.horse_bajikyo_id),
            horse_birthdate: data.horse_birthdate,
            horse_coat_color: data.horse_coat_color,
            horse_breed: data.horse_breed,
            breeder: data.horse_breeder,
            breeder_address: data.horse_breeder_address,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    let db_writer = horse_data
        .into_iter()
        .map(|x| DbWriter::BajikyoProfileToHorses(x))
        .collect::<Vec<_>>();

    write_to_db(&db_writer);
}
