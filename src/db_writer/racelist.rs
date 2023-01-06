use rusqlite::{Connection, params};
use super::Executer;

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

impl Executer for RaceListData{
    fn conn_execute(&self, conn: &Connection){
        conn.execute(
            "REPLACE  INTO races (
                race_id, race_date, racecourse, race_num, post_time,
                change, race_type, race_name,  surface, direction, 
                distance, weather, going, moisture, horse_count) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![
                self.race_id,
                self.race_date,
                self.racecourse,
                self.race_num,
                self.post_time,
                //
                self.change,
                self.race_type,
                self.race_name,
                self.surface,
                self.direction,
                //
                self.distance,
                self.weather,
                self.going,
                self.moisture,
                self.horse_count
                ],
        )
        .unwrap();
    }
}