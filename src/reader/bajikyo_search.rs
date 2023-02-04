use super::Reader;
use crate::common::horse_birthdate_parents::HorseBirthdateParents;
use base64::{engine::general_purpose, Engine as _};
use serde_json::{json, Value};

pub struct BajikyoSearchReader(HorseBirthdateParents);

enum Mode {
    Parents,
    Name,
    Sire,
    Dam,
}

impl BajikyoSearchReader {
    pub fn new(horse: HorseBirthdateParents) -> Self {
        Self(horse)
    }

    pub fn get(&self, is_force_fetch: bool, is_save: bool) {
        let text = self.get_string(is_force_fetch, is_save).unwrap();
        let v: Value = serde_json::from_str(&text).unwrap();

        if v["total"] != json!(1) {
            let text = self.get_string(true, is_save).unwrap();
            let v: Value = serde_json::from_str(&text).unwrap();
            if v["total"] != json!(1) {
                println!("!!!! {:?}", self.0);
            }
        }
    }
}

impl Reader for BajikyoSearchReader {
    fn get_url(&self) -> String {
        "".to_string()
    }

    fn fetch_string(&self) -> Option<String> {
        let original_data = make_query(&self.0, Mode::Parents);
        let text = send_req(&original_data);

        let v: Value = serde_json::from_str(&text).unwrap();
        if v["total"] == json!(1) {
            return Some(text);
        }

        let original_data = make_query(&self.0, Mode::Name);
        let text = send_req(&original_data);

        let v: Value = serde_json::from_str(&text).unwrap();
        if v["total"] == json!(1) {
            return Some(text);
        }

        let original_data = make_query(&self.0, Mode::Sire);
        let text = send_req(&original_data);

        let v: Value = serde_json::from_str(&text).unwrap();
        if v["total"] == json!(1) {
            return Some(text);
        }

        let original_data = make_query(&self.0, Mode::Dam);
        let text = send_req(&original_data);

        let v: Value = serde_json::from_str(&text).unwrap();
        if v["total"] == json!(1) {
            return Some(text);
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

    let client = reqwest::blocking::Client::new();
    std::thread::sleep(std::time::Duration::from_millis(3000));
    let res = client
        .post("https://www.bajikyo.or.jp/fg_list.php?listtype=202&pagetype=index")
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .body(format!(
            "page=1&rp=10&sortname=m_name&sortorder=asc&query={}&qtype=",
            encorded
        ))
        .send()
        .unwrap();
    let text = res.text().unwrap();
    text
}

fn make_query(data: &HorseBirthdateParents, mode: Mode) -> String {
    let birthdate: &str = &data.birthdate.format("%Y/%m/%d").to_string();
    let (sire_name, dam_name, subname): (&str, &str, &str) = match mode {
        Mode::Parents => (&data.sire_name, &data.dam_name, ""),
        Mode::Name => ("", "", &data.horse_name),
        Mode::Sire => (&data.sire_name, "", ""),
        Mode::Dam => ("", &data.dam_name, ""),
    };

    let original_data = String::new()
        + r#"a:22:{s:5:"assoc";s:1:"0";s:4:"name";s:0:"";s:3:"ph1";s:"#
        + &sire_name.as_bytes().len().to_string()
        + r#":""#
        + &sire_name
        + r#"";s:3:"ph2";s:"#
        + &dam_name.as_bytes().len().to_string()
        + r#":""#
        + &dam_name
        + r#"";s:5:"date1";s:"#
        + &birthdate.as_bytes().len().to_string()
        + r#":""#
        + &birthdate
        + r#"";s:5:"date2";s:"#
        + &birthdate.as_bytes().len().to_string()
        + r#":""#
        + &birthdate
        + r#"";s:6:"public";s:1:"1";s:5:"color";s:0:"";s:5:"breed";s:0:"";s:7:"breeder";s:0:"";s:5:"owner";s:0:"";s:3:"no1";s:0:"";s:3:"no2";s:0:"";s:2:"mc";s:0:"";s:7:"oldname";s:0:"";s:5:"subno";s:0:"";s:7:"subname";s:"#
        + &subname.as_bytes().len().to_string()
        + r#":""#
        + &subname
        + r#"";s:10:"savesubmit";s:10:"savesubmit";s:4:"page";s:1:"1";s:2:"rp";s:2:"40";s:8:"sortname";s:6:"m_name";s:9:"sortorder";s:3:"asc";}"#;
        original_data
}
