use anyhow::Result;
use chrono::{NaiveDate, NaiveTime};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

// 以下の構造体に基づいて、rusqliteでcreate tableする関数を作成してください。
// なお、サロゲートキーは使わず、optionのついていないメンバ変数を主キーにしてください。
// また、外部キー制約を用いてください。
// CREATE TABLE文は、IF NOT EXISTS を使ったものにしてください。
// テーブル名、およびカラム名はすべてスネークケースにしてください。

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Dates {
    pub date: NaiveDate,
    pub racecourse: Option<String>,
    pub fiscal_year: Option<i32>,
    pub kai: Option<String>,
    pub nichi: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Races {
    pub date: NaiveDate,
    pub race_num: i32,
    pub race_type: Option<String>,
    pub weather: Option<String>,
    pub going: Option<f64>,
    pub horse_count: Option<i32>,
    pub post_time: Option<NaiveTime>,
    pub post_time_change: Option<bool>,
    pub race_sub_name: Option<String>,
    pub race_name: Option<String>,
    pub race_weight_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RaceHorses {
    pub date: NaiveDate,
    pub race_num: i32,
    pub horse_num: i32,
    pub horse_nar_id: Option<i64>,
    pub bracket_num: Option<i32>,
    pub win_fav: Option<i32>,
    pub horse_weight: Option<i32>,
    pub jockey_id: Option<i32>,
    pub weight_to_carry: Option<i32>,
    pub trainer_id: Option<i32>,
    pub arrival: Option<i32>,
    pub arrival_info: Option<String>,
    pub finish_time: Option<f64>,
    pub prize: Option<i32>,
    pub change: Option<String>,
    pub horse_sex: Option<String>,
    pub weight_mark: Option<String>,
    pub owner_name: Option<String>,
    pub win_odds: Option<f64>,
    pub place_odds_min: Option<f64>,
    pub place_odds_max: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Horses {
    pub horse_nar_id: Option<i64>,
    pub horse_bajikyo_id: Option<String>,
    pub horse_name: Option<String>,
    pub horse_status: Option<String>,
    pub deregistration_date: Option<NaiveDate>,
    pub horse_birthdate: Option<NaiveDate>,
    pub horse_coat_color: Option<String>,
    pub horse_breed: Option<String>,
    pub breeder: Option<String>,
    pub breeder_address: Option<String>,
    pub sire_bajikyo_id: Option<String>,
    pub dam_bajikyo_id: Option<String>,
    pub bms_bajikyo_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Jockeys {
    pub jockey_nar_id: i64,
    pub name: String,
    pub kana: String,
    pub sex: String,
    pub status: String,
    pub birthdate: Option<NaiveDate>,
    pub first_run: Option<NaiveDate>,
    pub first_win: Option<NaiveDate>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Trainers {
    pub trainer_nar_id: i64,
    pub name: String,
    pub kana: String,
    pub sex: String,
    pub status: String,
    pub birthdate: Option<NaiveDate>,
    pub first_run: Option<NaiveDate>,
    pub first_win: Option<NaiveDate>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JockeyShortNames {
    pub jockey_short_name: String,
    pub jockey_nar_id: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrainerShortNames {
    pub trainer_short_name: String,
    pub trainer_nar_id: i64,
}

pub fn make_conn() -> Result<Connection> {
    let db_path = dirs::data_dir().unwrap().join("ukeiba").join("ukeiba.db");
    let conn = Connection::open(db_path)?;
    Ok(conn)
}

pub fn create_table() -> Result<()> {
    let conn = make_conn()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS dates (
            date DATE NOT NULL PRIMARY KEY,
            racecourse TEXT,
            fiscal_year INTEGER,
            kai TEXT,
            nichi TEXT
        )",
        params![],
    )?;

    // Racesテーブルの作成
    conn.execute(
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
    conn.execute(
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
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS horses (
            horse_nar_id INTEGER UNIQUE,
            horse_bajikyo_id TEXT UNIQUE,
            horse_name TEXT,
            horse_status TEXT,
            deregistration_date TEXT,
            horse_birthdate TEXT,
            horse_coat_color TEXT,
            horse_breed TEXT,
            breeder TEXT,
            breeder_address TEXT,
            sire_bajikyo_id TEXT,
            dam_bajikyo_id TEXT,
            bms_bajikyo_id TEXT
        )",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS jockeys (
            jockey_nar_id INTEGER PRIMARY KEY,
            name TEXT,
            kana TEXT,
            sex TEXT,
            status TEXT,
            birthdate DATE,
            first_run DATE,
            first_win DATE
        );
        ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS trainers (
            trainer_nar_id INTEGER PRIMARY KEY,
            name TEXT,
            kana TEXT,
            sex TEXT,
            status TEXT,
            birthdate DATE,
            first_run DATE,
            first_win DATE
        );
        ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS jockey_short_names (
            jockey_short_name TEXT PRIMARY KEY,
            jockey_nar_id INTEGER,
            FOREIGN KEY (jockey_nar_id) REFERENCES jockeys (jockey_nar_id)
        );
        ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS trainer_short_names (
            trainer_short_name TEXT PRIMARY KEY,
            trainer_nar_id INTEGER,
            FOREIGN KEY (trainer_nar_id) REFERENCES trainers (trainer_nar_id)
        );
        ",
        [],
    )?;

    Ok(())
}

pub fn vacuum_database() -> Result<()> {
    let conn = make_conn()?;
    conn.execute("VACUUM", [])?;
    Ok(())
}
