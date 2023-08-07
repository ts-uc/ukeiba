use anyhow::Result;
use chrono::{NaiveDate, NaiveTime};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
pub mod writer;

// 生成AI用コメント

// 以下の構造体に基づいて、SQLite でCREATE TABLE をするSQL文を作成してください。
// サロゲートキーは使わないでください。
// CREATE TABLE文は、IF NOT EXISTS を使ったものにしてください。
// テーブル名、およびカラム名はすべてスネークケースにしてください。
// 外部キー制約は、各テーブルのCREATE文の最後の方に、
// FOREIGN KEY (カラム名) REFERENCES テーブル名(カラム名) のようにして書いてください。
// NaiveDate、およびNaiveTime は TEXT型として扱ってください。

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Dates {
    pub race_date: NaiveDate, // 主キー
    pub racecourse: Option<String>,
    pub fiscal_year: Option<i32>,
    pub kai: Option<i32>,
    pub nichi: Option<i32>,
    pub capability_test: Option<i32>,
    pub heating: Option<bool>,
    pub sand_obstacle: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Races {
    pub race_date: NaiveDate, // 主キー・datesテーブルの外部キー
    pub race_num: i32,        // 主キー
    pub post_time: Option<NaiveTime>,
    pub post_time_change: Option<bool>,
    pub race_sub_name: Option<String>,
    pub race_name: Option<String>,
    pub race_detail: Option<String>,
    pub weather: Option<i32>,
    pub going: Option<f64>,
    pub race_class: Option<String>,
    pub race_kumi: Option<i32>,
    pub race_class_mixed: Option<bool>,
    pub race_kumi_mixed: Option<bool>,
    pub race_final: Option<bool>,
    pub race_age: Option<String>,
    pub race_sex: Option<String>,
    pub race_horse_select_type: Option<String>,
    pub race_weight_type: Option<i32>,
    pub race_type: Option<i32>,
    pub horse_count_run: Option<i32>,
    pub horse_count_entered: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RaceHorses {
    pub race_date: NaiveDate, // 主キー・racesテーブルの外部キー
    pub race_num: i32,        // 主キー・racesテーブルの外部キー
    pub horse_num: i32,       // 主キー
    pub horse_nar_id: Option<i64>,
    pub bracket_num: Option<i32>,
    pub gate_num: Option<i32>,
    pub horse_sex: Option<i32>,
    pub jockey_nar_id: Option<i32>,
    pub weight_mark: Option<i32>,
    pub weight_to_carry: Option<i32>,
    pub trainer_nar_id: Option<i32>,
    pub owner_name: Option<String>,
    pub horse_weight: Option<i32>,
    pub change: Option<String>,
    pub change_reason: Option<String>,
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
    pub horse_birth_year: Option<i32>,
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
    pub jockey_name: String,
    pub jockey_kana: String,
    pub jockey_sex: String,
    pub jockey_status: String,
    pub jockey_birthdate: Option<NaiveDate>,
    pub jockey_first_run: Option<NaiveDate>,
    pub jockey_first_win: Option<NaiveDate>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Trainers {
    pub trainer_nar_id: i32, // 主キー
    pub trainer_name: String,
    pub trainer_kana: String,
    pub trainer_sex: String,
    pub trainer_status: String,
    pub trainer_birthdate: Option<NaiveDate>,
    pub trainer_first_run: Option<NaiveDate>,
    pub trainer_first_win: Option<NaiveDate>,
}

pub fn make_conn() -> Result<Connection> {
    let db_path = dirs::data_dir().unwrap().join("ukeiba").join("ukeiba.db");
    let conn = Connection::open(db_path)?;
    Ok(conn)
}

pub fn create_table() -> Result<()> {
    let conn = make_conn()?;
    conn.execute_batch(
        //下記のようなCREATE文で定義される SQLITE DBがあります
        "
        CREATE TABLE IF NOT EXISTS dates (
            race_date TEXT PRIMARY KEY,
            racecourse TEXT,
            fiscal_year INTEGER,
            kai INTEGER,
            nichi INTEGER,
            capability_test INTEGER,
            heating INTEGER,
            sand_obstacle INTEGER      
        );
        
        CREATE TABLE IF NOT EXISTS races (
            race_date TEXT,
            race_num INTEGER,
            post_time TEXT,
            post_time_change INTEGER,
            race_sub_name TEXT,
            race_name TEXT,
            race_detail TEXT,
            weather INTEGER,
            going REAL,
            race_class TEXT,
            race_kumi INTEGER,
            race_class_mixed INTEGER,
            race_kumi_mixed INTEGER,
            race_final INTEGER,
            race_age TEXT,
            race_sex TEXT,
            race_horse_select_type TEXT,
            race_weight_type INTEGER,
            race_type INTEGER,
            horse_count_run INTEGER,
            horse_count_entered INTEGER,
            PRIMARY KEY (race_date, race_num),
            FOREIGN KEY (race_date) REFERENCES dates (race_date)
        );
        
        CREATE TABLE IF NOT EXISTS race_horses (
            race_date TEXT,
            race_num INTEGER,
            horse_num INTEGER,
            horse_nar_id INTEGER,
            bracket_num INTEGER,
            gate_num INTEGER,
            horse_sex INTEGER,
            jockey_nar_id INTEGER,
            weight_mark INTEGER,
            weight_to_carry INTEGER,
            trainer_nar_id INTEGER,
            owner_name TEXT,
            horse_weight INTEGER,
            change TEXT,
            change_reason TEXT,
            win_fav INTEGER,
            arrival INTEGER,
            arrival_info TEXT,
            finish_time REAL,
            prize INTEGER,
            win_odds REAL,
            place_odds_min REAL,
            place_odds_max REAL,
            PRIMARY KEY (race_date, race_num, horse_num),
            FOREIGN KEY (race_date, race_num) REFERENCES races (race_date, race_num)
        );
        CREATE INDEX IF NOT EXISTS idx_race_horses_horse_nar_id ON race_horses (horse_nar_id);

        CREATE TABLE IF NOT EXISTS horses (
            horse_bajikyo_id TEXT PRIMARY KEY,
            horse_nar_id INTEGER UNIQUE,
            horse_name TEXT,
            horse_status TEXT,
            deregistration_date TEXT,
            horse_birthdate TEXT,
            horse_birth_year INTEGER,
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
            jockey_name TEXT,
            jockey_kana TEXT,
            jockey_sex TEXT,
            jockey_status TEXT,
            jockey_birthdate TEXT,
            jockey_first_run TEXT,
            jockey_first_win TEXT
        );
        
        CREATE TABLE IF NOT EXISTS trainers (
            trainer_nar_id INTEGER PRIMARY KEY,
            trainer_name TEXT,
            trainer_kana TEXT,
            trainer_sex TEXT,
            trainer_status TEXT,
            trainer_birthdate TEXT,
            trainer_first_run TEXT,
            trainer_first_win TEXT
        );

        CREATE TABLE IF NOT EXISTS races_extend (
            race_date TEXT,
            race_num INTEGER,
            race_align INTEGER,
            PRIMARY KEY (race_date, race_num),
            FOREIGN KEY (race_date, race_num) REFERENCES races (race_date, race_num)
        );

        ",
    )?;

    Ok(())
}

pub fn update_race_align() -> Result<()> {
    let conn = make_conn()?;
    conn.execute_batch(
        "
        INSERT OR IGNORE INTO races_extend (race_date, race_num)
        SELECT race_date, race_num
        FROM races;

        UPDATE races_extend
        SET race_align = 
            CASE
                -- 2013年6月7日以前は常に「内詰め」
                WHEN races_extend.race_date <= '2013-06-07' THEN 0
                
                -- 2013年6月8日以降2014年10月24日以前
                WHEN races_extend.race_date <= '2014-10-24' THEN
                    CASE
                        -- nichiが1〜3のときは「内詰め」
                        WHEN dates.nichi BETWEEN 1 AND 3 THEN 0
                        -- nichiが4〜6のときは「外詰め」
                        WHEN dates.nichi BETWEEN 4 AND 6 THEN 1
                    END
                    
                -- 2014年10月25日以降
                ELSE
                    CASE
                        -- nichiが奇数かつrace_numが奇数、または、nichiが偶数かつrace_numが偶数のときは「内詰め」
                        WHEN (dates.nichi % 2 = 1 AND races_extend.race_num % 2 = 1) OR (dates.nichi % 2 = 0 AND races_extend.race_num % 2 = 0) THEN 0
                        -- nichiが奇数かつrace_numが偶数、または、nichiが偶数かつrace_numが奇数のときは「外詰め」
                        WHEN (dates.nichi % 2 = 1 AND races_extend.race_num % 2 = 0) OR (dates.nichi % 2 = 0 AND races_extend.race_num % 2 = 1) THEN 1
                    END
            END
        FROM dates
        WHERE dates.race_date = races_extend.race_date
            AND races_extend.race_align IS NULL; -- race_alignが未設定のレコードにのみ適用
        
        UPDATE race_horses
        SET gate_num = 
            CASE
                WHEN (SELECT race_align FROM races_extend WHERE races_extend.race_date = race_horses.race_date AND races_extend.race_num = race_horses.race_num) = 0 THEN race_horses.horse_num
                WHEN (SELECT race_align FROM races_extend WHERE races_extend.race_date = race_horses.race_date AND races_extend.race_num = race_horses.race_num) = 1 THEN 10 - (SELECT horse_count_entered FROM races WHERE races.race_date = race_horses.race_date AND races.race_num = race_horses.race_num) + race_horses.horse_num
                ELSE gate_num
            END
        WHERE gate_num IS NULL;
            ",
    )?;

    Ok(())
}

pub fn vacuum_database() -> Result<()> {
    let conn = make_conn()?;
    conn.execute("VACUUM", [])?;
    Ok(())
}
