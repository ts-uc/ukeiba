use super::WebPageTrait;
use crate::DbType;
use crate::{common::horse_birthdate_parents::HorseBirthdateParents, db_writer::Horses};
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone)]
pub struct BajikyoSearchPage(pub HorseBirthdateParents);

impl WebPageTrait for BajikyoSearchPage {
    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("bajikyo_search")
            .join(self.0.horse.get_upper_id().to_string())
            .join(format!(
                "bajikyo_search_{}.json.xz",
                self.0.horse.get_horse_id()
            ))
    }
    fn fetch_string(&self) -> Result<String> {
        if self.0.horse.get_horse_id() == 30892409283 {
            let query = SearchQuery {
                name: "".to_string(),
                subname: "".to_string(),
                sire_name: "".to_string(),
                dam_name: "".to_string(),
                birthdate: "".to_string(),
                bajikyo_id: "20101 2242".to_string(),
            };
            let text = send_req(&query.get())?;

            let v: Value = serde_json::from_str(&text).unwrap();
            if v["total"] == json!(1) && v["rows"].as_array().unwrap().len() == 1 {
                return Ok(text);
            }
        }

        for mode in Mode::iter() {
            let original_data = make_query(&self.0, mode);
            let text = send_req(&original_data)?;

            let v: Value = serde_json::from_str(&text).unwrap();
            if v["total"] == json!(1) && v["rows"].as_array().unwrap().len() == 1 {
                return Ok(text);
            }
        }

        Err(anyhow!("failed to narrow down"))
    }
    fn scrap(&self, body: &str) -> Result<Vec<DbType>> {
        let d: JsonData = serde_json::from_str(body).map_err(|e| anyhow!(e))?;
        let bajikyo_id = regex::Regex::new(r#"\&hno=(.+?)""#)
            .unwrap()
            .captures(&d.rows[0].cell[1])
            .unwrap()[1]
            .to_string();
        let data = DbType::BajikyoSearchHorses(Horses {
            horse_nar_id: Some(self.0.horse.get_horse_id()),
            horse_bajikyo_id: Some(bajikyo_id),
            ..Default::default()
        });
        Ok(vec![data])
    }
}

#[derive(Debug)]
struct SearchQuery {
    name: String,
    subname: String,
    sire_name: String,
    dam_name: String,
    birthdate: String,
    bajikyo_id: String,
}

#[derive(Debug, EnumIter)]
enum Mode {
    Parents,
    Subname,
    Name,
    Dam,
    Sire,
    DamSmall,
    SireSmall,
}

#[derive(Serialize, Deserialize)]
struct JsonData {
    page: String,
    rp: String,
    total: i32,
    rows: Vec<JsonDataRow>,
}

#[derive(Serialize, Deserialize)]
struct JsonDataRow {
    id: i32,
    cell: Vec<String>,
}

fn send_req(query: &str) -> Result<String> {
    let encorded = general_purpose::STANDARD.encode(query);
    let encorded = utf8_percent_encode(&encorded, NON_ALPHANUMERIC);
    let body = format!(
        "page=1&rp=10&sortname=m_name&sortorder=asc&query={}&qtype=",
        encorded
    );
    let client = reqwest::blocking::Client::new();
    std::thread::sleep(std::time::Duration::from_millis(3000));
    let res = client
        .post("https://www.bajikyo.or.jp/fg_list.php?listtype=202&pagetype=index")
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .body(body)
        .send()
        .map_err(|e| anyhow!(e))?
        .error_for_status()
        .map_err(|e| anyhow!(e))?;
    let text = res.text().map_err(|e| anyhow!(e))?;
    Ok(text)
}

fn make_query(data: &HorseBirthdateParents, mode: Mode) -> String {
    let birthdate = data.birthdate.format("%Y/%m/%d").to_string();
    let (name, sire_name, dam_name, subname): (String, String, String, String) = match mode {
        Mode::Parents => (
            "".to_string(),
            data.sire_name.to_string(),
            data.dam_name.to_string(),
            "".to_string(),
        ),
        Mode::Subname => (
            "".to_string(),
            "".to_string(),
            "".to_string(),
            data.horse_name.to_string(),
        ),
        Mode::Sire => (
            "".to_string(),
            data.sire_name.to_string(),
            "".to_string(),
            "".to_string(),
        ),
        Mode::Dam => (
            "".to_string(),
            "".to_string(),
            data.dam_name.to_string(),
            "".to_string(),
        ),
        Mode::Name => (
            data.horse_name.to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ),
        Mode::DamSmall => (
            "".to_string(),
            "".to_string(),
            smallize(&data.dam_name),
            "".to_string(),
        ),
        Mode::SireSmall => (
            "".to_string(),
            smallize(&data.sire_name),
            "".to_string(),
            "".to_string(),
        ),
    };

    let query = SearchQuery {
        name: name,
        subname: subname,
        sire_name: sire_name,
        dam_name: dam_name,
        birthdate: birthdate,
        bajikyo_id: "".to_string(),
    };

    query.get()
}

fn smallize(from: &str) -> String {
    from.replace("リユウ", "リュウ")
        .replace("フロンテイア", "フロンティア")
        .replace("ミツト", "ミット")
        .replace("レデイ", "レディ")
        .replace("ホツカイ", "ホッカイ")
}

impl SearchQuery {
    fn get(&self) -> String {
        String::new()
            + r#"a:22:{s:5:"assoc";s:1:"0";s:4:"name";s:"#
            + &self.name.as_bytes().len().to_string()
            + r#":""#
            + &self.name
            + r#"";s:3:"ph1";s:"#
            + &self.sire_name.as_bytes().len().to_string()
            + r#":""#
            + &self.sire_name
            + r#"";s:3:"ph2";s:"#
            + &self.dam_name.as_bytes().len().to_string()
            + r#":""#
            + &self.dam_name
            + r#"";s:5:"date1";s:"#
            + &self.birthdate.as_bytes().len().to_string()
            + r#":""#
            + &self.birthdate
            + r#"";s:5:"date2";s:"#
            + &self.birthdate.as_bytes().len().to_string()
            + r#":""#
            + &self.birthdate
            + r#"";s:6:"public";s:1:"1";s:5:"color";s:0:"";s:5:"breed";s:0:"";s:7:"breeder";s:0:"";s:5:"owner";s:0:"";s:3:"no1";s:"#
            + &self.bajikyo_id.as_bytes().len().to_string()
            + r#":""#
            + &self.bajikyo_id
            + r#"";s:3:"no2";s:0:"";s:2:"mc";s:0:"";s:7:"oldname";s:0:"";s:5:"subno";s:0:"";s:7:"subname";s:"#
            + &self.subname.as_bytes().len().to_string()
            + r#":""#
            + &self.subname
            + r#"";s:10:"savesubmit";s:10:"savesubmit";s:4:"page";s:1:"1";s:2:"rp";s:2:"10";s:8:"sortname";s:6:"m_name";s:9:"sortorder";s:3:"asc";}"#
    }
}
