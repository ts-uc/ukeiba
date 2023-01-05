//pub mod race;
// use crate::enums::Racecourse;
// use chrono::{Date, Local};
// use sqlx::sqlite::SqlitePool;
use rusqlite::params;
use rusqlite::Connection;
//use rusqlite::Statement;

#[derive(Debug)]
pub enum DbType {
    Initialize,
    RaceList(RaceListData),
}

#[derive(Debug)]
pub struct RaceListData {
    pub race_id: i64,
    pub race_date: String,
    pub racecourse: String,
    pub race_num: i32,
    pub post_time: Option<String>,

    pub change: Option<String>,
    pub race_type: Option<String>,
    pub race_name: Option<String>,
    pub surface: Option<String>,
    pub direction: Option<String>,

    pub distance: Option<i32>,
    pub weather: Option<String>,
    pub going: Option<String>,
    pub moisture: Option<f64>,
    pub horse_count: Option<i32>,
}

pub struct Db(Vec<DbType>);

impl Db {
    pub fn new(data: Vec<DbType>) -> Self {
        Db(data)
    }

    pub fn debug(&self) {
        println!("{:?}", self.0);
    }

    pub fn execute(&self) {
        if self.0.is_empty() {
            return;
        }
        let db_path = dirs::data_dir().unwrap().join("ukeiba").join("ukeiba.db");
        let conn = Connection::open(&db_path).unwrap();
        for db_type in &self.0 {
            match db_type {
                DbType::Initialize => {
                    conn.execute(
                        "
                        CREATE TABLE IF NOT EXISTS races (
                            race_id INTEGER PRIMARY KEY,
                            race_date TEXT NOT NULL,
                            racecourse TEXT NOT NULL,
                            race_num INTEGER NOT NULL,
                            post_time TEXT,
                        
                            change TEXT,
                            race_type TEXT,
                            race_name TEXT,
                            surface TEXT,               
                            direction TEXT,
                        
                            distance INTEGER,
                            weather TEXT,
                            going TEXT,
                            moisture REAL,
                            horse_count INTEGER,
                        
                            created_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime')),
                            updated_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime'))
                        );
                        
                        CREATE TRIGGER IF NOT EXISTS trigger_races_updated_at AFTER UPDATE ON races
                        BEGIN
                            UPDATE races SET updated_at = DATETIME('now', 'localtime') WHERE rowid == NEW.rowid;
                        END;
                        
                        CREATE TABLE IF NOT EXISTS race_horses (
                            race_horse_id INTEGER PRIMARY KEY,
                            race_id INTEGER NOT NULL,
                            horse_num INTEGER NOT NULL,
                            bracket_num INTEGER,
                            arrival INTEGER,
                            horse_id INTEGER,
                            horse_sex TEXT,
                            horse_age INTEGER,
                            weight_to_carry INTEGER,
                            jockey_id INTEGER,
                            trainer_id INTEGER,
                            horse_weight INTEGER,
                            horse_weight_delta INTEGER,
                            finish_time REAL,
                            margin_time REAL,
                            margin TEXT,
                            last_3f REAL,
                            win_fav INTEGER,
                            win_odds REAL,
                            place_odds_min REAL,
                            place_odds_max REAL,
                            prize INTEGER,
                            created_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime')),
                            updated_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime'))
                        );
                        
                        CREATE TRIGGER IF NOT EXISTS trigger_race_horses_updated_at AFTER UPDATE ON race_horses
                        BEGIN
                            UPDATE race_horses SET updated_at = DATETIME('now', 'localtime') WHERE rowid == NEW.rowid;
                        END;
                        
                        CREATE TABLE IF NOT EXISTS horses (
                            horse_id INTEGER PRIMARY KEY,
                            horse_name TEXT,
                            sire_name TEXT,
                            dam_name TEXT,
                            sires_sire_name TEXT,
                            sires_dam_name TEXT,
                            dams_sire_name TEXT,
                            dams_dam_name TEXT,
                            breeder TEXT,
                            birthplace TEXT,
                            created_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime')),
                            updated_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime'))
                        );
                        
                        CREATE TRIGGER IF NOT EXISTS trigger_horses_updated_at AFTER UPDATE ON horses
                        BEGIN
                            UPDATE horses SET updated_at = DATETIME('now', 'localtime') WHERE rowid == NEW.rowid;
                        END;
                        
                        CREATE TABLE IF NOT EXISTS jockeys (
                            jockey_id INTEGER PRIMARY KEY,
                            jockey_name TEXT,
                            jockey_sex TEXT,
                            jockey_status TEXT,
                            jockey_affiliation TEXT,
                            created_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime')),
                            updated_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime'))
                        );
                        
                        CREATE TRIGGER IF NOT EXISTS trigger_jockeys_updated_at AFTER UPDATE ON jockeys
                        BEGIN
                            UPDATE jockeys SET updated_at = DATETIME('now', 'localtime') WHERE rowid == NEW.rowid;
                        END;
                        
                        CREATE TABLE IF NOT EXISTS trainers (
                            trainer_id INTEGER PRIMARY KEY,
                            trainer_name TEXT,
                            trainer_sex TEXT,
                            trainer_status TEXT,
                            trainer_affiliation TEXT,
                            created_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime')),
                            updated_at TEXT NOT NULL DEFAULT (DATETIME('now', 'localtime'))
                        );
                        
                        CREATE TRIGGER IF NOT EXISTS trigger_trainers_updated_at AFTER UPDATE ON trainers
                        BEGIN
                            UPDATE trainers SET updated_at = DATETIME('now', 'localtime') WHERE rowid == NEW.rowid;
                        END;
                        ",
                        ()
                    ).unwrap();
                },
                DbType::RaceList(data) => {
                    conn.execute(
                        "REPLACE  INTO races (
                            race_id, race_date, racecourse, race_num, post_time,
                            change, race_type, race_name,  surface, direction, 
                            distance, weather, going, moisture, horse_count) 
                            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
                        params![
                            data.race_id,
                            data.race_date,
                            data.racecourse,
                            data.race_num,
                            data.post_time,
                            //
                            data.change,
                            data.race_type,
                            data.race_name,
                            data.surface,
                            data.direction,
                            //
                            data.distance,
                            data.weather,
                            data.going,
                            data.moisture,
                            data.horse_count
                            ],
                    )
                    .unwrap();
                }
            }
        }
    }
}
