use super::Reader;
use crate::common::horse_birthdate_parents::HorseBirthdateParents;
use base64::{engine::general_purpose, Engine as _};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde_json::{json, Value};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct BajikyoSearchReader(HorseBirthdateParents);

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

impl BajikyoSearchReader {
    pub fn new(horse: HorseBirthdateParents) -> Self {
        Self(horse)
    }

    pub fn get(&self, is_force_fetch: bool, is_save: bool) {
        let text = self.get_string(is_force_fetch, is_save).unwrap();
        let v: Value = serde_json::from_str(&text).unwrap();

        if !v["rows"].is_array() && v["rows"].as_array().unwrap().len() != 1 {
            self.get_string(true, is_save).unwrap();
        }
    }
}

impl Reader for BajikyoSearchReader {
    fn get_url(&self) -> String {
        "".to_string()
    }

    fn fetch_string(&self) -> Option<String> {
        if self.0.horse.get_horse_id() == 30892409283 {
            let query = SearchQuery {
                name: "".to_string(),
                subname: "".to_string(),
                sire_name: "".to_string(),
                dam_name: "".to_string(),
                birthdate: "".to_string(),
                bajikyo_id: "20101 2242".to_string(),
            };
            let text = send_req(&query.get());

            let v: Value = serde_json::from_str(&text).unwrap();
            println!("{}", v["total"]);
            if v["total"] == json!(1) {
                return Some(text);
            }
        }

        for mode in Mode::iter() {
            let original_data = make_query(&self.0, mode);
            let text = send_req(&original_data);

            let v: Value = serde_json::from_str(&text).unwrap();
            println!("{}", v["total"]);
            if v["total"] == json!(1) {
                return Some(text);
            }
        }

        Some(r"{}".to_string())
    }

    fn get_file_dir_path(&self) -> std::path::PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("bajikyo_search")
            .join(self.0.horse.get_upper_id().to_string())
    }

    fn get_file_path(&self) -> std::path::PathBuf {
        self.get_file_dir_path().join(format!(
            "bajikyo_search_{}.json.gz",
            self.0.horse.get_horse_id()
        ))
    }
}

fn send_req(query: &str) -> String {
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
        .unwrap();
    let text = res.text().unwrap();
    text
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
