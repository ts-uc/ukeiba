use anyhow::Result;
use chrono::{NaiveDate, NaiveTime};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
pub mod writer;

// 以下の構造体に基づいて、SQLITE でCREATE TABLE をするSQL文を作成してください。
// なお、サロゲートキーは使わないでください。
// CREATE TABLE文は、IF NOT EXISTS を使ったものにしてください。
// テーブル名、およびカラム名はすべてスネークケースにしてください。

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Dates {
    pub date: NaiveDate, // 主キー
    pub racecourse: Option<String>,
    pub fiscal_year: Option<i32>,
    pub kai: Option<i32>,
    pub nichi: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Races {
    pub date: NaiveDate, // 主キー・datesテーブルの外部キー
    pub race_num: i32,   // 主キー
    pub post_time: Option<NaiveTime>,
    pub post_time_change: Option<bool>,
    pub race_sub_name: Option<String>,
    pub race_name: Option<String>,
    pub weather: Option<String>,
    pub going: Option<f64>,
    pub race_weight_type: Option<String>,
    pub race_type: Option<String>,
    pub horse_count_run: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RaceHorses {
    pub date: NaiveDate, // 主キー・racesテーブルの外部キー
    pub race_num: i32,   // 主キー・racesテーブルの外部キー
    pub horse_num: i32,  // 主キー
    pub horse_nar_id: Option<i64>,
    pub bracket_num: Option<i32>,
    pub horse_sex: Option<String>,
    pub jockey_nar_id: Option<i32>,
    pub weight_mark: Option<String>,
    pub weight_to_carry: Option<i32>,
    pub trainer_nar_id: Option<i32>,
    pub owner_name: Option<String>,
    pub horse_weight: Option<i32>,
    pub change: Option<String>,
    pub win_fav: Option<i32>,
    pub arrival: Option<i32>,
    pub arrival_info: Option<String>,
    pub finish_time: Option<f64>,
    pub prize: Option<i32>,
    pub win_odds: Option<f64>,
    pub place_odds_min: Option<f64>,
    pub place_odds_max: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Horses {
    pub horse_bajikyo_id: Option<String>, // 主キー
    pub horse_nar_id: Option<i64>,        // ユニークキー
    pub horse_name: Option<String>,
    pub horse_status: Option<String>,
    pub deregistration_date: Option<NaiveDate>,
    pub horse_birthdate: Option<NaiveDate>,
    pub horse_coat_color: Option<String>,
    pub horse_breed: Option<String>,
    pub breeder: Option<String>,
    pub breeder_location: Option<String>,
    pub sire_bajikyo_id: Option<String>,
    pub dam_bajikyo_id: Option<String>,
    pub bms_bajikyo_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Jockeys {
    pub jockey_nar_id: i32, // 主キー
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
    pub trainer_nar_id: i32, // 主キー
    pub name: String,
    pub kana: String,
    pub sex: String,
    pub status: String,
    pub birthdate: Option<NaiveDate>,
    pub first_run: Option<NaiveDate>,
    pub first_win: Option<NaiveDate>,
}

pub fn make_conn() -> Result<Connection> {
    let db_path = dirs::data_dir().unwrap().join("ukeiba").join("ukeiba.db");
    let conn = Connection::open(db_path)?;
    Ok(conn)
}

pub fn create_table() -> Result<()> {
    let conn = make_conn()?;
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS dates (
            date TEXT PRIMARY KEY,
            racecourse TEXT,
            fiscal_year INTEGER,
            kai INTEGER,
            nichi INTEGER
        );
        
        CREATE TABLE IF NOT EXISTS races (
            date TEXT,
            race_num INTEGER,
            post_time TEXT,
            post_time_change INTEGER,
            race_sub_name TEXT,
            race_name TEXT,
            weather TEXT,
            going REAL,
            race_weight_type TEXT,
            race_type TEXT,
            horse_count_run INTEGER,
            PRIMARY KEY (date, race_num),
            FOREIGN KEY (date) REFERENCES dates(date)
        );
        
        CREATE TABLE IF NOT EXISTS race_horses (
            date TEXT,
            race_num INTEGER,
            horse_num INTEGER,
            horse_nar_id INTEGER,
            bracket_num INTEGER,
            horse_sex TEXT,
            jockey_nar_id INTEGER,
            weight_mark TEXT,
            weight_to_carry INTEGER,
            trainer_nar_id INTEGER,
            owner_name TEXT,
            horse_weight INTEGER,
            change TEXT,
            win_fav INTEGER,
            arrival INTEGER,
            arrival_info TEXT,
            finish_time REAL,
            prize INTEGER,
            win_odds REAL,
            place_odds_min REAL,
            place_odds_max REAL,
            PRIMARY KEY (date, race_num, horse_num),
            FOREIGN KEY (date, race_num) REFERENCES races(date, race_num)
        );
        
        CREATE TABLE IF NOT EXISTS horses (
            horse_bajikyo_id TEXT PRIMARY KEY,
            horse_nar_id INTEGER UNIQUE,
            horse_name TEXT,
            horse_status TEXT,
            deregistration_date TEXT,
            horse_birthdate TEXT,
            horse_coat_color TEXT,
            horse_breed TEXT,
            breeder TEXT,
            breeder_location TEXT,
            sire_bajikyo_id TEXT,
            dam_bajikyo_id TEXT,
            bms_bajikyo_id TEXT
        );
        
        CREATE TABLE IF NOT EXISTS jockeys (
            jockey_nar_id INTEGER PRIMARY KEY,
            name TEXT,
            kana TEXT,
            sex TEXT,
            status TEXT,
            birthdate TEXT,
            first_run TEXT,
            first_win TEXT
        );
        
        CREATE TABLE IF NOT EXISTS trainers (
            trainer_nar_id INTEGER PRIMARY KEY,
            name TEXT,
            kana TEXT,
            sex TEXT,
            status TEXT,
            birthdate TEXT,
            first_run TEXT,
            first_win TEXT
        );
        ",
    )?;

    Ok(())
}

pub fn vacuum_database() -> Result<()> {
    let conn = make_conn()?;
    conn.execute("VACUUM", [])?;
    Ok(())
}
