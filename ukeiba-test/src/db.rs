use anyhow::Result;
use chrono::{NaiveDate, NaiveTime};
use rusqlite::{params, Connection, Transaction};
use serde::{Deserialize, Serialize};
use serde_rusqlite::to_params_named;

// 以下の構造体に基づいて、rusqliteでcreate tableする関数を作成してください。
// なお、サロゲートキーは使わず、optionのついていないメンバ変数を主キーにしてください。
// また、外部キー制約を用いてください。
// CREATE TABLE文は、IF NOT EXISTS を使ったものにしてください。
// テーブル名、およびカラム名はすべてスネークケースにしてください。

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Dates {
    date: NaiveDate,
    racecourse: Option<String>,
    kai: Option<String>,
    nichi: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

pub fn make_conn() -> Result<Connection> {
    let db_path = dirs::data_dir().unwrap().join("ukeiba").join("ukeiba.db");
    let conn = Connection::open(db_path)?;
    Ok(conn)
}

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn new() -> Result<Self> {
        let conn = make_conn()?;
        Ok(Db { conn })
    }

    pub fn create_table(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS dates (
                date DATE NOT NULL PRIMARY KEY,
                racecourse TEXT,
                kai TEXT,
                nichi TEXT
            )",
            params![],
        )?;

        // Racesテーブルの作成
        self.conn.execute(
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
        self.conn.execute(
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
        self.conn.execute(
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

        Ok(())
    }

    pub fn vacuum_database(&self) -> Result<()> {
        self.conn.execute("VACUUM", [])?;
        Ok(())
    }
}

pub fn upsert_dates(transaction: &Transaction, dates: &Dates) -> Result<()> {
    transaction.execute(
        "
        INSERT INTO dates (date, racecourse, kai, nichi)
        VALUES (:date, :racecourse, :kai, :nichi)
        ON CONFLICT(date) DO UPDATE SET
            racecourse = COALESCE(:racecourse, dates.racecourse),
            kai = COALESCE(:kai, dates.kai),
            nichi = COALESCE(:nichi, dates.nichi)
    ",
        to_params_named(&dates)?.to_slice().as_slice(),
    )?;
    Ok(())
}

pub fn upsert_horses(transaction: &Transaction, horses: &Horses) -> Result<()> {
    transaction.execute(
    "
    INSERT INTO horses (horse_nar_id, horse_birthdate, horse_name, horse_status, horse_type, deregistration_date)
    VALUES (:horse_nar_id, :horse_birthdate, :horse_name, :horse_status, :horse_type, :deregistration_date)
    ON CONFLICT(horse_nar_id) DO UPDATE SET
    horse_birthdate = COALESCE(:horse_birthdate, horses.horse_birthdate),
    horse_name = COALESCE(:horse_name, horses.horse_name),
    horse_status = COALESCE(:horse_status, horses.horse_status),
    horse_type = COALESCE(:horse_type, horses.horse_type),
    deregistration_date = COALESCE(:deregistration_date, horses.deregistration_date)
    ",
    to_params_named(&horses)?.to_slice().as_slice(),
    )?;
    Ok(())
}
