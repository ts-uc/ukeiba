use super::*;
use anyhow::Result;
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use base64::{engine::general_purpose, Engine};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Query {
    pub assoc: String,
    pub name: String,
    pub ph1: String,
    pub ph2: String,
    pub date1: String,
    pub date2: String,
    pub public: String,
    pub color: String,
    pub breed: String,
    pub breeder: String,
    pub owner: String,
    pub no1: String,
    pub no2: String,
    pub mc: String,
    pub oldname: String,
    pub subno: String,
    pub subname: String,
    pub savesubmit: String,
    pub page: String,
    pub rp: String,
    pub sortname: String,
    pub sortorder: String,
}

#[derive(Debug, Clone, serde::Serialize)]
struct RequestData {
    page: String,
    rp: String,
    sortname: String,
    sortorder: String,
    query: String,
    qtype: String,
}

impl Default for Query {
    fn default() -> Self {
        Query {
            assoc: "0".to_string(),
            name: "".to_string(),
            ph1: "".to_string(),
            ph2: "".to_string(),
            date1: "1901/01/01".to_string(),
            date2: "1901/01/01".to_string(),
            public: "1".to_string(),
            color: "".to_string(),
            breed: "".to_string(),
            breeder: "".to_string(),
            owner: "".to_string(),
            no1: "".to_string(),
            no2: "".to_string(),
            mc: "".to_string(),
            oldname: "".to_string(),
            subno: "".to_string(),
            subname: "".to_string(),
            savesubmit: "savesubmit".to_string(),
            page: "1".to_string(),
            rp: "10".to_string(),
            sortname: "m_name".to_string(),
            sortorder: "asc".to_string(),
        }
    }
}

impl Default for RequestData {
    fn default() -> Self {
        RequestData {
            page: "1".to_string(),
            rp: "10".to_string(),
            sortname: "m_name".to_string(),
            sortorder: "asc".to_string(),
            query: "".to_string(),
            qtype: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Data {
    pub hits: i32,
    pub horse_nar_id_list: Vec<i64>,
}

impl WebPageTrait for Query {
    type Data = Data;

    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("fetched")
            .join("bajikyo_search")
            .join(format!(
                "{}_{}_{}_{}_{}.json.xz",
                self.name, self.ph1, self.ph2, self.date1, self.date2
            ))
    }
    fn fetch_string(&self, interval: Duration) -> Result<String> {
        std::thread::sleep(interval);
        let request_data = make_request_data(&self)?;
        let client = reqwest::blocking::Client::new();
        let res = client
            .post("https://www.bajikyo.or.jp/fg_list.php?listtype=202&pagetype=index")
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(request_data)
            .send()?
            .error_for_status()?;
        let text = res.text().map_err(|e| anyhow!(e))?;
        Ok(text)
    }
    fn scrap_string(&self, body: &str) -> Result<Self::Data> {
        todo!()
    }
}

fn make_request_data(query: &Query) -> Result<String> {
    let serialized = serde_php::to_vec(&query)?;
    let encoded = general_purpose::STANDARD.encode(&serialized);
    let request_data = RequestData {
        query: encoded,
        ..Default::default()
    };
    let request_data = serde_qs::to_string(&request_data)?;
    Ok(request_data)
}
