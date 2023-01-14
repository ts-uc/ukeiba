#![allow(unused)]
use rusqlite::{params, Connection};

fn get_conn() -> Connection {
    let db_path = dirs::data_dir().unwrap().join("ukeiba").join("ukeiba.db");
    Connection::open(&db_path).unwrap()
}

pub fn initialize() {
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

#[derive(Debug)]
pub struct Races{
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
    pub horse_count: Option<String>,
}

#[derive(Debug)]
pub struct RaceHorses{
    pub race_horse_id: i64,
    pub race_id: i64,
    pub horse_num: i32,
    pub bracket_num: Option<String>,
    pub horse_name: Option<String>,
    pub horse_sex: Option<String>,
    pub horse_age: Option<String>,
    pub horse_id: Option<i64>,
    pub jockey_name: Option<String>,
    pub jockey_id: Option<i32>,
    pub trainer_name: Option<String>,
    pub trainer_id: Option<i32>,
    pub change: Option<String>,
    pub owner_name: Option<String>,
    pub weight_mark: Option<String>,
    pub weight_to_carry: Option<String>,
    pub horse_weight: Option<String>,
    pub horse_weight_delta: Option<String>,
    pub arrival: Option<String>,
    pub finish_time: Option<String>,
    pub margin_time: Option<String>,
    pub margin: Option<String>,
    pub last_3f: Option<String>,
    pub win_fav: Option<String>,
    pub win_odds: Option<String>,
    pub place_odds_min: Option<String>,
    pub place_odds_max: Option<String>,
    pub prize: Option<String>,
}


#[derive(Debug)]
pub enum DbType {
    RaceList(Races),
    Race(RaceHorses),
    HorseHistoryRace(Races),
    HorseHistoryRaceHorse(RaceHorses),
}

pub struct Db(Vec<DbType>);

impl Db {
    pub fn new(data: Vec<DbType>) -> Self {
        Db(data)
    }

    pub fn execute(&self) {
        if self.0.is_empty() {
            return;
        }
        let mut conn = get_conn();
        let tx = conn.transaction().unwrap();
        let pb = indicatif::ProgressBar::new(self.0.len() as u64);
        for db_type in &self.0 {
            match db_type {
                DbType::RaceList(data) => {
                    tx.execute(
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
                DbType::Race(data) => {
                    tx.execute(
                        "REPLACE  INTO race_horses (
                            race_horse_id, race_id, horse_num,  horse_name,
                            horse_sex,  horse_id, jockey_name, jockey_id, 
                            trainer_name, trainer_id, change, owner_name, weight_mark,
                            weight_to_carry, horse_weight) 
                            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
                        params![
                            data.race_horse_id,
                            data.race_id,
                            data.horse_num,
                            data.horse_name,
                            //
                            data.horse_sex,
                            data.horse_id,
                            data.jockey_name,
                            data.jockey_id,
                            //
                            data.trainer_name,
                            data.trainer_id,
                            data.change,
                            data.owner_name,
                            data.weight_mark,
                            //
                            data.weight_to_carry,
                            data.horse_weight,
                            ],
                    )
                    .unwrap();
                }
                DbType::HorseHistoryRace(data) => {
                    tx.execute(
                        "INSERT OR IGNORE INTO races (
                            race_id, race_date, racecourse, race_num, change, 
                            race_type, race_name,  surface, distance, weather, 
                            going, moisture, horse_count) 
                            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                        params![
                            data.race_id,
                            data.race_date,
                            data.racecourse,
                            data.race_num,
                            data.change,
                            //
                            data.race_type,
                            data.race_name,
                            data.surface,
                            data.distance,
                            data.weather,
                            //
                            data.going,
                            data.moisture,
                            data.horse_count
                        ],
                    )
                    .unwrap();
                }
                DbType::HorseHistoryRaceHorse(data) => {
                    tx.execute(
                        "INSERT INTO race_horses (
                            race_horse_id, race_id, bracket_num, horse_num, win_fav,
                            arrival, finish_time, margin_time, last_3f, horse_weight, 
                            jockey_name, weight_to_carry, trainer_name, prize, horse_id, horse_name
                            ) 
                            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)
                            ON CONFLICT (race_horse_id) DO UPDATE SET
                            race_id = ?2, bracket_num = ?3, horse_num = ?4, win_fav = ?5,
                            arrival = ?6, finish_time = ?7, margin_time = ?8, last_3f = ?9,
                            prize = ?14",
                    params![
                        data.race_horse_id,
                        data.race_id,
                        data.bracket_num,
                        data.horse_num,
                        data.win_fav,
                        //
                        data.arrival,
                        data.finish_time,
                        data.margin_time,
                        data.last_3f,
                        data.horse_weight,
                        //
                        data.jockey_name,
                        data.weight_to_carry,
                        data.trainer_name,
                        data.prize,
                        data.horse_id,
                        //
                        data.horse_name,
                        ],
                )
                .unwrap();
                }
            }
            pb.inc(1);
        }
        tx.commit().unwrap();
    }
}
