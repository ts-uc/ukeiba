//pub mod race;
// use crate::enums::Racecourse;
// use chrono::{Date, Local};
// use sqlx::sqlite::SqlitePool;

#[derive(Debug)]
pub enum DbType{
    RaceList(RaceListData)
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

    pub distance : Option<i32>,
    pub weather: Option<String>,
    pub going: Option<String>,
    pub moisture: Option<f64>,
    pub horse_count : Option<i32>,
}

pub struct Db(Vec<DbType>);

impl Db{
    pub fn new(data: Vec<DbType>) -> Self{
        Db(data)
    }

    pub fn debug(&self){
        println!("{:?}", self.0);
    }
    // pub async fn add(&self, pool: &SqlitePool) -> Option<()> {
    //     println!("{:?}",self.0);
    //     return Some(());
    //     let mut conn = pool.acquire().await.ok()?;
    
    //     sqlx::query!(
    //         r#"
    // INSERT INTO todos ( description )
    // VALUES ( ?1 )
    //         "#,
    //         description
    //     )
    //     .execute(&mut conn)
    //     .await?;
    
    //     Some(())
    // }
    
}