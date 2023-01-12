use rusqlite::{Connection, params};
use super::Executer;

#[derive(Debug)]
pub struct HorseHistoryRaceHorse {
    pub racehorse_id: i64,
    pub race_id: i64,
    pub bracket_num: i32,
    pub horse_num: i32,
    pub win_fav: Option<String>,

    pub arrival: Option<String>,
    pub finish_time: Option<String>,
    pub margin_time: Option<String>,
    pub last_3f: Option<String>,
    pub horse_weight: Option<String>,

    pub jockey_name: Option<String>,
    pub weight_to_carry: Option<String>,
    pub trainer_name: Option<String>,
    pub prize: Option<String>,
    pub horse_id: i64,

    //    pub horse_name: String,

}

impl Executer for HorseHistoryRaceHorse{
    fn conn_execute(&self, conn: &Connection){
        conn.execute(
                "INSERT INTO race_horses (
                    race_horse_id, race_id, bracket_num, horse_num, win_fav,
                    arrival, finish_time, margin_time, last_3f, horse_weight, 
                    jockey_name, weight_to_carry, trainer_name, prize, horse_id,
                    ) 
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
                    ON CONFLICT(racehorse_id) DO UPDATE SET
                    race_id = ?2, bracket_num = ?3, horse_num = ?4, win_fav = ?5,
                    arrival = ?6, finish_time = ?7, margin_time = ?8, last_3f = ?9, horse_weight = ?10,
                    jockey_name = ?11, weight_to_carry = ?12, trainer_name = ?13, prize = ?14, horse_id = ?15",
    
            params![
                self.racehorse_id,
                self.race_id,
                self.bracket_num,
                self.horse_num,
                self.win_fav,
                //
                self.arrival,
                self.finish_time,
                self.margin_time,
                self.last_3f,
                self.horse_weight,
                //
                self.jockey_name,
                self.weight_to_carry,
                self.trainer_name,
                self.prize,
                self.horse_id,
                ],
        )
        .unwrap();
    }
}