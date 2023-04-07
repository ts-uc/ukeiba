use super::*;
use crate::common::trainer::Trainer;
use crate::db_writer::DbType;
use crate::db_writer::Trainers;
use anyhow::{bail, Result};
use chrono::NaiveDate;
use scraper::Html;
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone)]
pub struct TrainerPage(pub Trainer);

impl WebPageTrait for TrainerPage {
    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("trainer")
            .join(format!("trainer_{}.html.xz", self.0.get_id()))
    }
    fn fetch_string(&self) -> Result<String> {
        let url = format!(
            "https://www.keiba.go.jp/KeibaWeb/DataRoom/TrainerMark?k_trainerLicenseNo={}",
            self.0.get_id()
        );
        let got_string = get_from_url(&url)?;
        if !got_string.contains("html") {
            bail!("required tag is not exist");
        }
        Ok(got_string)
    }
    fn scrap(&self, body: &str) -> Result<Vec<DbType>> {
        let document: String = body.nfkc().collect();
        if document.contains("ありません") {
            return Ok(Vec::new());
        }
        let document = Html::parse_document(&document);

        let mut data: Vec<DbType> = Vec::new();

        let inner_data = Trainers {
            trainer_id: self.0.get_id(),
            trainer_name: scrap(&document, ".horseinfo > li:nth-child(1) > h4:nth-child(1)")
                .map(|s| remove_whitespace(&s))
                .unwrap_or("".to_string()),
            trainer_sex: scrap(&document, ".sex").unwrap_or("".to_string()),
            trainer_status: scrap(&document, ".horseinfo > li:nth-child(4) > div:nth-child(1)")
                .unwrap_or("".to_string()),
            trainer_affiliation: scrap(
                &document,
                ".trainerinfo > tbody:nth-child(1) > tr:nth-child(1) > td:nth-child(2)",
            )
            .unwrap_or("".to_string()),
            trainer_birthdate: scrap(
                &document,
                ".trainerinfo > tbody:nth-child(1) > tr:nth-child(2) > td:nth-child(2)",
            )
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y/%m/%d").ok())
            .map(|d| d.to_string()),
            trainer_first_run_date: scrap(
                &document,
                ".trainerinfo > tbody:nth-child(1) > tr:nth-child(3) > td:nth-child(2)",
            )
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y/%m/%d").ok())
            .map(|d| d.to_string()),
            trainer_first_win_date: scrap(
                &document,
                ".trainerinfo > tbody:nth-child(1) > tr:nth-child(4) > td:nth-child(2)",
            )
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y/%m/%d").ok())
            .map(|d| d.to_string()),
        };

        data.push(DbType::TrainerHeader(inner_data));

        Ok(data)
    }
}