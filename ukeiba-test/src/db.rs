use chrono::{NaiveDate, NaiveTime};
use rusqlite::{params, Connection, Result};

// 以下の構造体に基づいて、rusqliteでcreate tableする関数を作成してください。
// なお、サロゲートキーは使わず、optionのついていないメンバ変数を主キーにしてください。
// また、外部キー制約を用いてください。
// CREATE TABLE文は、IF NOT EXISTS を使ったものにしてください。
// テーブル名、およびカラム名はすべてスネークケースにしてください。

#[derive(Debug, Clone, Default)]
pub struct Dates {
    date: NaiveDate,
    racecourse: Option<String>,
    kai: Option<String>,
    nichi: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Races {
    date: NaiveDate,
    race_num: i32,
    race_type: Option<String>,
    weather: Option<String>,
    going: Option<f64>,
    horse_count: Option<i32>,
    post_time: Option<NaiveTime>,
    post_time_change: Option<bool>,
    race_sub_name: Option<String>,
    race_name: Option<String>,
    race_weight_type: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RaceHorses {
    date: NaiveDate,
    race_num: i32,
    horse_num: i32,
    horse_nar_id: Option<i64>,
    bracket_num: Option<i32>,
    win_fav: Option<i32>,
    horse_weight: Option<i32>,
    jockey_id: Option<i32>,
    weight_to_carry: Option<i32>,
    trainer_id: Option<i32>,
    arrival: Option<i32>,
    arrival_info: Option<String>,
    finish_time: Option<f64>,
    prize: Option<i32>,
    change: Option<String>,
    horse_sex: Option<String>,
    weight_mark: Option<String>,
    owner_name: Option<String>,
    win_odds: Option<f64>,
    place_odds_min: Option<f64>,
    place_odds_max: Option<f64>,
}

#[derive(Debug, Clone, Default)]
pub struct Horses {
    horse_nar_id: i64,
    horse_birthdate: Option<NaiveDate>,
    horse_name: Option<String>,
    horse_status: Option<String>,
    horse_type: Option<String>,
    deregistration_date: Option<NaiveDate>,
}

pub fn create_table(connection: &Connection) -> Result<()> {
    // Datesテーブルの作成
    connection.execute(
        "CREATE TABLE IF NOT EXISTS dates (
            date DATE NOT NULL PRIMARY KEY,
            racecourse TEXT,
            kai TEXT,
            nichi TEXT
        )",
        params![],
    )?;

    // Racesテーブルの作成
    connection.execute(
        "CREATE TABLE IF NOT EXISTS races (
            date DATE NOT NULL,
            race_num INTEGER NOT NULL,
            race_type TEXT,
            weather TEXT,
            going REAL,
            horse_count INTEGER,
            post_time TEXT,
            post_time_change INTEGER,
            race_sub_name TEXT,
            race_name TEXT,
            race_weight_type TEXT,
            PRIMARY KEY (date, race_num),
            FOREIGN KEY (date) REFERENCES dates (date)
        )",
        params![],
    )?;

    // RaceHorsesテーブルの作成
    connection.execute(
        "CREATE TABLE IF NOT EXISTS race_horses (
            date DATE NOT NULL,
            race_num INTEGER NOT NULL,
            horse_num INTEGER NOT NULL,
            horse_nar_id INTEGER,
            bracket_num INTEGER,
            win_fav INTEGER,
            horse_weight INTEGER,
            jockey_id INTEGER,
            weight_to_carry INTEGER,
            trainer_id INTEGER,
            arrival INTEGER,
            arrival_info TEXT,
            finish_time REAL,
            prize INTEGER,
            change TEXT,
            horse_sex TEXT,
            weight_mark TEXT,
            owner_name TEXT,
            win_odds REAL,
            place_odds_min REAL,
            place_odds_max REAL,
            PRIMARY KEY (date, race_num, horse_num),
            FOREIGN KEY (date, race_num) REFERENCES races (date, race_num)
        )",
        params![],
    )?;

    // Horsesテーブルの作成
    connection.execute(
        "CREATE TABLE IF NOT EXISTS horses (
            horse_nar_id INTEGER NOT NULL PRIMARY KEY,
            horse_birthdate DATE,
            horse_name TEXT,
            horse_status TEXT,
            horse_type TEXT,
            deregistration_date DATE
        )",
        params![],
    )?;

    Ok(())
}

pub fn vacuum_database(connection: &Connection) -> Result<()> {
    connection.execute("VACUUM", [])?;
    Ok(())
}
