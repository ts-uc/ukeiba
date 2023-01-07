use rusqlite::Connection;

fn get_conn() -> Connection{
    let db_path = dirs::data_dir().unwrap().join("ukeiba").join("ukeiba.db");
    Connection::open(&db_path).unwrap()
}

pub fn get_racelist() -> Vec<i64>{
    let conn = get_conn();
    let mut stmt = conn.prepare("SELECT race_id FROM races").unwrap();
    let data_iter= stmt.query_map([], |row| row.get(0)).unwrap();
    let mut data = Vec::new();
    for d in data_iter{
        data.push(d.unwrap());
    }
    data
}
