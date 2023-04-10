use super::*;
use anyhow::{bail, Context, Result};
use chrono::NaiveDate;
use scraper::Html;
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone)]
pub struct Page {
    pub horse_nar_id: i64,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub horse_name: String,
    pub horse_sex: String,
    pub horse_status: String,
    pub horse_type: Option<String>,
    pub deregistration_date: Option<NaiveDate>,
    pub data: Vec<DataRow>,
}

#[derive(Debug, Clone)]
pub struct DataRow {
    pub race_date: NaiveDate,
    pub racecourse: String,
    pub race_num: i32,
    pub race_type: Option<String>,
    pub race_name: Option<String>,
    pub race_class: Option<String>,
    pub distance: Option<i32>,
    pub weather: Option<String>,
    pub going: Option<String>,
    pub night_race: Option<String>,
    pub horse_count: Option<i32>,
    pub bracket_num: Option<i32>,
    pub horse_num: Option<i32>,
    pub win_fav: Option<i32>,
    pub arrival: Option<i32>,
    pub arrival_raw: Option<String>,
    pub finish_time: Option<f64>,
    pub margin_time: Option<f64>,
    pub last_3f: Option<f64>,
    pub horse_weight: Option<i32>,
    pub jockey_name: Option<String>,
    pub jockey_affiliation: Option<String>,
    pub weight_to_carry: Option<i32>,
    pub trainer_name: Option<String>,
    pub prize: Option<i32>,
    pub win_horse_name: Option<String>,
}

impl WebPageTrait for Page {
    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("fetched")
            .join("horse_history")
            .join(format!("{}.html.xz", self.horse_nar_id.to_string()))
    }
    fn fetch_string(&self, interval: Duration) -> Result<String> {
        let url = format!(
            "https://www.keiba.go.jp/KeibaWeb/DataRoom/HorseMarkInfo?k_lineageLoginCode={}&k_activeCode=1",
            self.horse_nar_id
        );
        let got_string = get_from_url(&url, interval)?;
        if !got_string.contains("html") {
            bail!("required tag is not exist");
        }
        Ok(got_string)
    }
}

impl WebPage<Page> {
    pub fn scrap(&self) -> Result<Data> {
        let doc: String = self.body.nfkc().collect();
        let doc = Html::parse_document(&doc);
        let doc = doc.root_element();

        let mut data = Vec::new();

        for element in
            doc.select(&Selector::parse(".HorseMarkInfo_table > tbody:nth-child(2) > tr").unwrap())
        {
            let grade_color = element
                .select(&Selector::parse("td:nth-child(4)").unwrap())
                .next()
                .and_then(|e| e.value().attr("class"));
            let race_type = match grade_color {
                Some("green") => "特別",
                Some("yellow") => "準重賞",
                Some("pink") => "重賞",
                _ => "一般",
            };
            data.push(DataRow {
                race_date: scrap(&element, "td:nth-child(1)")
                    .and_then(|s| NaiveDate::parse_from_str(&s, "%Y/%m/%d").ok())
                    .unwrap_or_default(),
                racecourse: scrap(&element, "td:nth-child(2)").unwrap_or_default(),
                race_num: scrap(&element, "td:nth-child(3)")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_default(),
                race_type: Some(race_type.to_string()),
                race_name: scrap_remove_tag(&element, "a:nth-child(1) > p:nth-child(1)"),
                race_class: scrap_remove_tag(&element, "td:nth-child(5)"),
                distance: scrap(&element, "td:nth-child(6)").and_then(|s| s.parse().ok()),
                weather: scrap_remove_tag(&element, "td:nth-child(7)"),
                going: scrap_remove_tag(&element, "td:nth-child(8)"),
                night_race: scrap_remove_tag(&element, "td:nth-child(9)"),
                horse_count: scrap(&element, "td:nth-child(10)").and_then(|s| s.parse().ok()),
                bracket_num: scrap(&element, "td:nth-child(11)").and_then(|s| s.parse().ok()),
                horse_num: scrap(&element, "td:nth-child(12)").and_then(|s| s.parse().ok()),
                win_fav: scrap(&element, "td:nth-child(13)").and_then(|s| s.parse().ok()),
                arrival: scrap(&element, "td:nth-child(14)").and_then(|s| s.parse().ok()),
                arrival_raw: scrap(&element, "td:nth-child(14)"),
                finish_time: scrap(&element, "td:nth-child(15)").and_then(|s| convert_time(&s)),
                margin_time: scrap(&element, "td:nth-child(16)").and_then(|s| s.parse().ok()),
                last_3f: scrap(&element, "td:nth-child(17)").and_then(|s| s.parse().ok()),
                horse_weight: scrap(&element, "td:nth-child(18)").and_then(|s| s.parse().ok()),
                jockey_name: scrap(&element, "td:nth-child(19)")
                    .map(|s| split_bracket(&s).0.to_string()),
                jockey_affiliation: scrap(&element, "td:nth-child(19)")
                    .map(|s| split_bracket(&s).1.to_string()),
                weight_to_carry: scrap(&element, "td:nth-child(20)").and_then(|s| s.parse().ok()),
                trainer_name: scrap(&element, "td:nth-child(21)"),
                prize: scrap(&element, "td:nth-child(22)")
                    .and_then(|s| s.replace(",", "").parse().ok()),
                win_horse_name: scrap(&element, "td:nth-child(23)"),
            });
        }

        Ok(Data {
            horse_name: scrap(&doc, ".odd_title").context("essential error")?,
            horse_sex: scrap(&doc, ".sex").context("essential error")?,
            horse_status: scrap(&doc, ".horseinfo > li:nth-child(3) > div:nth-child(1)")
                .context("essential error")?,
            horse_type: scrap(&doc, ".horseinfo > li:nth-child(4) > div:nth-child(1)"),
            deregistration_date: scrap(&doc, ".annotationRed")
                .and_then(|s| NaiveDate::parse_from_str(&s, "※%Y/%m/%d 地方競馬登録抹消").ok()),
            data: data,
        })
    }
}
