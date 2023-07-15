use ukeiba_common::common::HorseBajikyoId;

use crate::db::make_conn;

pub fn get_from_db() -> Vec<HorseBajikyoId> {
    let conn = make_conn().unwrap();

    // horse_bajikyo_idを取得するクエリ
    let query = "SELECT horse_bajikyo_id FROM horses";

    // クエリを実行し、結果を取得
    let mut stmt = conn.prepare(query).unwrap();
    let rows = stmt.query_map([], |row| row.get(0)).unwrap();

    // horse_bajikyo_idの値をVec<String>に格納
    let horse_bajikyo_ids = rows
        .map(|row| HorseBajikyoId(row.unwrap()))
        .collect::<Vec<_>>();

    horse_bajikyo_ids
}
