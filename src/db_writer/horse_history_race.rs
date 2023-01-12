use rusqlite::{Connection, params};
use super::Executer;

#[derive(Debug)]
pub struct HorseHistoryRaceData {
    pub race_id: i64,
    pub race_date: String,
    pub racecourse: String,
    pub race_num: i32,
    pub change: Option<String>,

    pub race_type: Option<String>,
    pub race_name: Option<String>,
    pub surface: Option<String>,
    pub distance: Option<String>,
    pub weather: Option<String>,

    pub going: Option<String>,
    pub moisture: Option<String>,
    pub horse_count: Option<String>,
}

impl Executer for HorseHistoryRaceData{
    fn conn_execute(&self, conn: &Connection){
        conn.execute(
            "INSERT OR IGNORE INTO races (
                race_id, race_date, racecourse, race_num, change, 
                race_type, race_name,  surface, distance, weather, 
                going, moisture, horse_count) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                self.race_id,
                self.race_date,
                self.racecourse,
                self.race_num,
                self.change,
                //
                self.race_type,
                self.race_name,
                self.surface,
                self.distance,
                self.weather,
                //
                self.going,
                self.moisture,
                self.horse_count
            ],
        )
        .unwrap();
    }
}