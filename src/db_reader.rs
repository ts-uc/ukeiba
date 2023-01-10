use chrono::NaiveDate;
use rusqlite::Connection;

fn get_conn() -> Connection {
    let db_path = dirs::data_dir().unwrap().join("ukeiba").join("ukeiba.db");
    Connection::open(&db_path).unwrap()
}

pub fn get_racelist(from: NaiveDate, to: NaiveDate) -> Vec<i64> {
    let conn = get_conn();
    let sql = format!(
        "SELECT race_id FROM races where '{}' <= race_date and race_date <= '{}'",
        from.to_string(),
        to.to_string()
    );
    let mut stmt = conn.prepare(&sql).unwrap();
    let data_iter = stmt.query_map([], |row| row.get(0)).unwrap();
    let mut data = Vec::new();
    for d in data_iter {
        data.push(d.unwrap());
    }
    data
}

pub fn get_horselist(from: NaiveDate, to: NaiveDate) -> Vec<i64> {
    let conn = get_conn();
    let sql = format!("select distinct horse_id from races inner join race_horses on races.race_id = race_horses.race_id where '{}' <= race_date and race_date <= '{}'", from.to_string(), to.to_string());
    let mut stmt = conn.prepare(&sql).unwrap();
    let data_iter = stmt.query_map([], |row| row.get(0)).unwrap();
    let mut data = Vec::new();
    for d in data_iter {
        data.push(d.unwrap());
    }
    data
}
