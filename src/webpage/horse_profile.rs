use super::*;
use crate::common::horse::Horse;
use crate::db_writer::DbType;
use crate::db_writer::Horses;
use crate::NaiveDate;
use scraper::{Html, Selector};
use unicode_normalization::UnicodeNormalization;

#[derive(Debug)]
pub struct PageHorseProfile {
    pub html: String,
    pub horse: Horse,
}

impl PageHorseProfile {
    pub fn new(html: String, horse: Horse) -> Self {
        Self {
            html: html,
            horse: horse,
        }
    }

    pub fn db(&self) -> Vec<DbType> {
        let document: String = self.html.nfkc().collect();
        let document = Html::parse_document(&document);

        let mut data: Vec<DbType> = Vec::new();

        let horse_data = Horses {
            horse_nar_id: Some(self.horse.get_horse_id()),
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
            birthplace: scrap(&document, "td.notopborder:nth-child(2)"),
            breeder: scrap(&document, "td.notopborder:nth-child(4)")
            .map(|s| s.split_whitespace().collect::<Vec<_>>().join("")),
            sire_name: scrap(&document, ".fathername"),
            dam_name: scrap(&document, ".pedigree > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child(3) > td:nth-child(2)"),
            sires_sire_name: scrap(&document, ".Paternalfathername"),
            sires_dam_name: scrap(&document, ".pedigree > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child(2) > td:nth-child(2)"),
            dams_sire_name: scrap(&document, ".pedigree > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child(3) > td:nth-child(4)"),
            dams_dam_name: scrap(&document, ".pedigree > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child(4) > td:nth-child(2)"),
            deregistration_date: None,
        };

        data.push(DbType::HorseProfile(horse_data));

        data
    }
}

fn scrap(html: &Html, selector_str: &str) -> Option<String> {
    let selector = Selector::parse(&selector_str).unwrap();
    let text = scrap_text(&html, &selector);
    text.filter(|s| !s.is_empty())
}
