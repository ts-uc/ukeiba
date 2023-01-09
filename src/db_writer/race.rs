use rusqlite::{Connection, params};
use super::Executer;

#[derive(Debug)]
pub struct RaceData {
    pub racehorse_id: i64,
    pub race_id: i64,
    pub horse_num: i32,
    pub bracket_num: i32,
    pub horse_name: String,

    pub horse_sex: String,
    pub horse_age: i32,
    pub horse_id: Option<i64>,
    pub jockey_name: String,
    pub jockey_id: Option<i64>,

    pub trainer_name: String,
    pub trainer_id: Option<i64>,
    pub change: String,
    pub owner_name: String,
    pub weight_mark: String,

    pub weight_to_carry: String,
    pub horse_weight: String
}

impl Executer for RaceData{
    fn conn_execute(&self, conn: &Connection){
        conn.execute(
            "REPLACE  INTO race_horses (
                race_horse_id, race_id, horse_num, bracket_num, horse_name,
                horse_sex, horse_age, horse_id, jockey_name, jockey_id, 
                trainer_name, trainer_id, change, owner_name, weight_mark,
                weight_to_carry, horse_weight) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
            params![
                self.racehorse_id,
                self.race_id,
                self.horse_num,
                self.bracket_num,
                self.horse_name,
                //
                self.horse_sex,
                self.horse_age,
                self.horse_id,
                self.jockey_name,
                self.jockey_id,
                //
                self.trainer_name,
                self.trainer_id,
                self.change,
                self.owner_name,
                self.weight_mark,
                //
                self.weight_to_carry,
                self.horse_weight,
                ],
        )
        .unwrap();
    }
}