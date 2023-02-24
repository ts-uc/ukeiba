use super::*;
use crate::common::horse::Horse;
use crate::db_writer::DbType;
use crate::db_writer::Horses;
use crate::NaiveDate;
use anyhow::Result;
use scraper::Html;
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone)]
pub struct HorseProfilePage(pub Horse);

impl WebPageTrait for HorseProfilePage {
    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("horse_profile")
            .join(self.0.get_upper_id().to_string())
            .join(format!("horse_profile_{}.html.gz", self.0.get_horse_id()))
    }
    fn fetch_string(&self) -> Result<String> {
        let url = format!(
            "https://www.keiba.go.jp/KeibaWeb/DataRoom/RaceHorseInfo?k_lineageLoginCode={}&k_activeCode=1",
            self.0.get_horse_id()
        );
        get_from_url(&url)
    }
    fn scrap(&self, body: &str) -> Vec<DbType> {
        let document: String = body.nfkc().collect();
        let document = Html::parse_document(&document);

        let mut data: Vec<DbType> = Vec::new();

        let horse_data = Horses {
            horse_nar_id: Some(self.0.get_horse_id()),
            horse_name: scrap(&document, ".odd_title"),
            horse_sex: scrap(&document, ".sex"),
            horse_status: scrap(&document, ".horseinfo > li:nth-child(3) > div:nth-child(1)"),
            horse_type: scrap(&document, ".horseinfo > li:nth-child(4) > div:nth-child(1)")
                .and_then(|s| detect_inner_bracket(&s)),
            horse_birthdate: scrap(
                &document,
                ".horse_info_table > tbody:nth-child(2) > tr:nth-child(1) > td:nth-child(2)",
            ).and_then(|s| NaiveDate::parse_from_str(&s, "%Y.%m.%d生").ok()).map(|d| d.to_string()),
            horse_coat_color: scrap(
                &document,
                ".horse_info_table > tbody:nth-child(2) > tr:nth-child(2) > td:nth-child(2)",
            ),
            birthplace: scrap(&document, "td.notopborder:nth-child(2)").map(|s| remove_whitespace(&s)),
            breeder: scrap(&document, "td.notopborder:nth-child(4)")
            .map(|s| remove_whitespace(&s)),
            sire_name: scrap(&document, ".fathername").and_then(|s| detect_after_bracket(&s)).map(|s| remove_whitespace(&s)),
            dam_name: scrap(&document, ".pedigree > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child(3) > td:nth-child(2)").and_then(|s| detect_after_bracket(&s)).map(|s| remove_whitespace(&s)),
            sires_sire_name: scrap(&document, ".Paternalfathername").and_then(|s| detect_after_bracket(&s)).map(|s| remove_whitespace(&s)),
            sires_dam_name: scrap(&document, ".pedigree > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child(2) > td:nth-child(2)").and_then(|s| detect_after_bracket(&s)).map(|s| remove_whitespace(&s)),
            dams_sire_name: scrap(&document, ".pedigree > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child(3) > td:nth-child(4)").and_then(|s| detect_after_bracket(&s)).map(|s| remove_whitespace(&s)),
            dams_dam_name: scrap(&document, ".pedigree > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child(4) > td:nth-child(2)").and_then(|s| detect_after_bracket(&s)).map(|s| remove_whitespace(&s)),
            ..Default::default()
        };

        data.push(DbType::HorseProfile(horse_data));

        data
    }
}
