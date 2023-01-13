#![allow(unused)]
pub mod racelist;
pub mod race;
pub mod horse_history_race;
pub mod horse_history_racehorse;
use rusqlite::Connection;
use crate::db_writer::racelist::RaceListData;
use crate::db_writer::race::RaceData;

use self::horse_history_race::HorseHistoryRaceData;
use self::horse_history_racehorse::HorseHistoryRaceHorse;

fn get_conn() -> Connection{
    let db_path = dirs::data_dir().unwrap().join("ukeiba").join("ukeiba.db");
    Connection::open(&db_path).unwrap()
}

pub fn initialize(){
    let conn = get_conn();
    conn.execute_batch(
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
            horse_name TEXT,
            horse_sex TEXT,
            horse_age INTEGER,
            horse_id INTEGER,
            jockey_name TEXT,
            jockey_id INTEGER,
            trainer_name TEXT,
            trainer_id INTEGER,
            change TEXT,
            owner_name TEXT,
            weight_mark TEXT,
            weight_to_carry INTEGER,
            horse_weight INTEGER,
            horse_weight_delta INTEGER,
            arrival INTEGER,
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
    ).unwrap();
}

pub trait Executer{
    fn conn_execute(&self, conn: &Connection);
    fn execute(&self){
        let conn = get_conn();
        self.conn_execute(&conn)
    }
}

#[derive(Debug)]
pub enum DbType {
    RaceList(RaceListData),
    Race(RaceData),
    HorseHistoryRace(HorseHistoryRaceData),
    HorseHistoryRaceHorse(HorseHistoryRaceHorse)
}

impl Executer for DbType{
    fn conn_execute(&self, conn: &Connection){
        match self {
            DbType::RaceList(data) => {
                data.conn_execute(&conn);
            }
            DbType::Race(data) => {
                data.conn_execute(&conn)
            }
            DbType::HorseHistoryRace(data) => {
                data.conn_execute(&conn)
            }
            DbType::HorseHistoryRaceHorse(data) => {
                data.conn_execute(&conn)
            }
        }
    }
}

pub struct Db(Vec<DbType>);

impl Db {
    pub fn new(data: Vec<DbType>) -> Self {
        Db(data)
    }

    pub fn debug(&self) {
        println!("{:?}", self.0);
    }
}

impl Executer for Db{
    fn conn_execute(&self, conn: &Connection) {
        if self.0.is_empty() {
            return;
        }
        for db_type in &self.0 {
            db_type.conn_execute(&conn)
        }
    }

    fn execute(&self) {
        if self.0.is_empty() {
            return;
        }
        let conn = get_conn();
        let pb = indicatif::ProgressBar::new(self.0.len() as u64);
        for db_type in &self.0 {
            db_type.conn_execute(&conn);
            pb.inc(1);
        }
    }
}
