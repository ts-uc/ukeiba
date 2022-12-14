#![allow(unused)]
pub mod racelist;
pub mod race;
pub mod initialize;
pub mod horse_history_race;
pub mod horse_history_racehorse;
use rusqlite::Connection;
use crate::db_writer::racelist::RaceListData;
use crate::db_writer::race::RaceData;
use crate::db_writer::initialize::Initialize;

use self::horse_history_race::HorseHistoryRaceData;
use self::horse_history_racehorse::HorseHistoryRaceHorse;

pub trait Executer{
    fn conn_execute(&self, conn: &Connection);
    fn get_conn(&self) -> Connection{
        let db_path = dirs::data_dir().unwrap().join("ukeiba").join("ukeiba.db");
        Connection::open(&db_path).unwrap()
    }
    fn execute(&self){
        let conn = self.get_conn();
        self.conn_execute(&conn)
    }
}

#[derive(Debug)]
pub enum DbType {
    Initialize,
    RaceList(RaceListData),
    Race(RaceData),
    HorseHistoryRace(HorseHistoryRaceData),
    HorseHistoryRaceHorse(HorseHistoryRaceHorse)
}

impl Executer for DbType{
    fn conn_execute(&self, conn: &Connection){
        match self {
            DbType::Initialize => {
                Initialize::new().conn_execute(&conn);
            },
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
        let conn = self.get_conn();
        for db_type in &self.0 {
            db_type.conn_execute(&conn)
        }
    }
}
