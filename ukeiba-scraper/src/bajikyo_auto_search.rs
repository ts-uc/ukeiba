use super::*;
use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, serde::Serialize)]
pub struct OriginalData {
    pub horse_nar_id: i64,
    pub horse_name: String,
    pub birthdate: NaiveDate,
    pub sire_name: String,
    pub dam_name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct FileData {
    pub horse_bajikyo_id: Option<String>,
    pub fetch_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Data {
    pub horse_nar_id: i64,
    pub horse_name: String,
    pub birthdate: NaiveDate,
    pub sire_name: String,
    pub dam_name: String,
    pub horse_bajikyo_id: Option<String>,
    pub fetch_mode: Option<String>,
}

impl WebPageTrait for OriginalData {
    type Data = Data;

    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("fetched")
            .join("bajikyo_auto_search")
            .join(format!(
                "{}_{}_{}_{}_{}.json.xz",
                self.horse_nar_id,
                self.horse_name,
                self.birthdate.clone(),
                self.sire_name.clone(),
                self.dam_name.clone()
            ))
    }

    fn fetch_string(&self, interval: Duration) -> Result<String> {
        //ID
        let query = bajikyo_search::Query {
            no1: to_bajikyo_id(self.horse_nar_id),
            date1: self.birthdate.format("%Y/%m/%d").to_string(),
            date2: self.birthdate.format("%Y/%m/%d").to_string(),
            ..Default::default()
        };
        let data = query.scrap_string(&query.fetch_string(interval)?)?;
        if data.hits == 1 {
            let file_data = FileData {
                horse_bajikyo_id: Some(data.horse_bajikyo_id_list[0].clone()),
                fetch_mode: Some("ID".to_string()),
            };
            return Ok(serde_json::to_string(&file_data)?);
        }

        //父馬名・母馬名
        let query = bajikyo_search::Query {
            ph1: self.sire_name.clone(),
            ph2: self.dam_name.clone(),
            date1: self.birthdate.format("%Y/%m/%d").to_string(),
            date2: self.birthdate.format("%Y/%m/%d").to_string(),
            ..Default::default()
        };
        let data = query.scrap_string(&query.fetch_string(interval)?)?;
        if data.hits == 1 {
            let file_data = FileData {
                horse_bajikyo_id: Some(data.horse_bajikyo_id_list[0].clone()),
                fetch_mode: Some("parents".to_string()),
            };
            return Ok(serde_json::to_string(&file_data)?);
        }

        //血統名
        let query = bajikyo_search::Query {
            name: self.horse_name.clone(),
            date1: self.birthdate.format("%Y/%m/%d").to_string(),
            date2: self.birthdate.format("%Y/%m/%d").to_string(),
            ..Default::default()
        };
        let data = query.scrap_string(&query.fetch_string(interval)?)?;
        if data.hits == 1 {
            let file_data = FileData {
                horse_bajikyo_id: Some(data.horse_bajikyo_id_list[0].clone()),
                fetch_mode: Some("bajikyo_name".to_string()),
            };
            return Ok(serde_json::to_string(&file_data)?);
        }

        //母馬名
        let query = bajikyo_search::Query {
            ph2: self.dam_name.clone(),
            date1: self.birthdate.format("%Y/%m/%d").to_string(),
            date2: self.birthdate.format("%Y/%m/%d").to_string(),
            ..Default::default()
        };
        let data = query.scrap_string(&query.fetch_string(interval)?)?;
        if data.hits == 1 {
            let file_data = FileData {
                horse_bajikyo_id: Some(data.horse_bajikyo_id_list[0].clone()),
                fetch_mode: Some("dam_name".to_string()),
            };
            return Ok(serde_json::to_string(&file_data)?);
        }

        //父馬名
        let query = bajikyo_search::Query {
            ph1: self.sire_name.clone(),
            date1: self.birthdate.format("%Y/%m/%d").to_string(),
            date2: self.birthdate.format("%Y/%m/%d").to_string(),
            ..Default::default()
        };
        let data = query.scrap_string(&query.fetch_string(interval)?)?;
        if data.hits == 1 {
            let file_data = FileData {
                horse_bajikyo_id: Some(data.horse_bajikyo_id_list[0].clone()),
                fetch_mode: Some("sire_name".to_string()),
            };
            return Ok(serde_json::to_string(&file_data)?);
        }

        //母馬名
        let query = bajikyo_search::Query {
            ph2: convert_name(&self.dam_name),
            date1: self.birthdate.format("%Y/%m/%d").to_string(),
            date2: self.birthdate.format("%Y/%m/%d").to_string(),
            ..Default::default()
        };
        let data = query.scrap_string(&query.fetch_string(interval)?)?;
        if data.hits == 1 {
            let file_data = FileData {
                horse_bajikyo_id: Some(data.horse_bajikyo_id_list[0].clone()),
                fetch_mode: Some("dam_name_converted".to_string()),
            };
            return Ok(serde_json::to_string(&file_data)?);
        }

        //父馬名
        let query = bajikyo_search::Query {
            ph1: convert_name(&self.sire_name),
            date1: self.birthdate.format("%Y/%m/%d").to_string(),
            date2: self.birthdate.format("%Y/%m/%d").to_string(),
            ..Default::default()
        };
        let data = query.scrap_string(&query.fetch_string(interval)?)?;
        if data.hits == 1 {
            let file_data = FileData {
                horse_bajikyo_id: Some(data.horse_bajikyo_id_list[0].clone()),
                fetch_mode: Some("sire_name_converted".to_string()),
            };
            return Ok(serde_json::to_string(&file_data)?);
        }

        //NAR登録名
        let query = bajikyo_search::Query {
            subname: self.horse_name.clone(),
            ..Default::default()
        };
        let data = query.scrap_string(&query.fetch_string(interval)?)?;
        if data.hits == 1 {
            let file_data = FileData {
                horse_bajikyo_id: Some(data.horse_bajikyo_id_list[0].clone()),
                fetch_mode: Some("nar_name".to_string()),
            };
            return Ok(serde_json::to_string(&file_data)?);
        }

        //見つからなかったとき
        let file_data = FileData {
            horse_bajikyo_id: None,
            fetch_mode: Some("not_found".to_string()),
        };
        return Ok(serde_json::to_string(&file_data)?);
    }

    fn scrap_string(&self, body: &str) -> Result<Self::Data> {
        let file_data: FileData = serde_json::from_str(body)?;
        let data = Data {
            horse_nar_id: self.horse_nar_id,
            horse_name: self.horse_name.clone(),
            birthdate: self.birthdate,
            sire_name: self.sire_name.clone(),
            dam_name: self.dam_name.clone(),
            horse_bajikyo_id: file_data.horse_bajikyo_id,
            fetch_mode: file_data.fetch_mode,
        };
        println!("{:?}", data);
        Ok(data)
    }
}

fn to_bajikyo_id(nar_id: i64) -> String {
    match nar_id {
        30892409283 => return "20101 2242".to_string(),
        _ => (),
    }

    let chars: Vec<char> = nar_id.to_string().chars().collect();
    let shuffled: i64 = format!(
        "{}{}{}{}{}{}{}{}{}{}",
        chars[5],
        chars[1],
        chars[10],
        chars[9],
        chars[2],
        chars[0],
        chars[4],
        chars[8],
        chars[3],
        chars[7]
    )
    .parse()
    .unwrap();
    let mut num_chars: Vec<char> = (shuffled - 2046971875).to_string().chars().rev().collect();

    if num_chars.len() >= 5 {
        if num_chars[4] == '5' {
            num_chars[4] = ' ';
        } else if num_chars[4] == '4' {
            num_chars[4] = 'H';
        }
    }

    num_chars.iter().rev().collect()
}

fn convert_name(from: &str) -> String {
    from.replace("リユウ", "リュウ")
        .replace("フロンテイア", "フロンティア")
        .replace("ミツト", "ミット")
        .replace("レデイ", "レディ")
        .replace("ホツカイ", "ホッカイ")
}
