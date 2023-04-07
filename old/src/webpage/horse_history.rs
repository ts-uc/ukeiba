use super::*;
use crate::common::date_racecourse::DateRacecourse;
use crate::common::horse::Horse;
use crate::common::race::Race;
use crate::common::race_horse::RaceHorse;
use crate::common::racecourse::Racecourse;
use crate::db_writer::DateRacecourses;
use crate::db_writer::DbType;
use crate::db_writer::RaceHorses;
use crate::db_writer::Races;
use anyhow::{bail, Result};
use chrono::NaiveDate;
use scraper::{Html, Selector};
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone)]
pub struct HorseHistoryPage(pub Horse);

impl WebPageTrait for HorseHistoryPage {
    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("horse_history")
            .join(self.0.get_upper_id().to_string())
            .join(format!("horse_history_{}.html.xz", self.0.get_horse_id()))
    }
    fn fetch_string(&self) -> Result<String> {
        let url = format!(
            "https://www2.keiba.go.jp/KeibaWeb/DataRoom/HorseMarkInfo?k_lineageLoginCode={}",
            self.0.get_horse_id()
        );
        let got_string = get_from_url(&url)?;
        if !got_string.contains("html") {
            bail!("required tag is not exist");
        }
        Ok(got_string)
    }
    fn scrap(&self, body: &str) -> Result<Vec<DbType>> {
        let document: String = body.nfkc().collect();
        let document = Html::parse_document(&document);

        let row_selector = ".HorseMarkInfo_table > tbody:nth-child(2) > tr";
        let row_selector = Selector::parse(row_selector).unwrap();
        let column_selector = "td";
        let column_selector = Selector::parse(column_selector).unwrap();
        let horse_name_selector = ".odd_title";
        let horse_name_selector = Selector::parse(horse_name_selector).unwrap();

        let scrapped = scrap_grid(&document, &row_selector, &column_selector);
        let horse_name = document
            .select(&horse_name_selector)
            .next()
            .unwrap()
            .inner_html();

        let mut data = Vec::new();

        for (i, scrapped_row) in scrapped.iter().enumerate() {
            let race_name_selector = Selector::parse(&format!(
                ".HorseMarkInfo_table > tbody:nth-child(2) > tr:nth-child({}) > td:nth-child(4)",
                i + 1
            ))
            .unwrap();
            let selected = document
                .select(&race_name_selector)
                .next()
                .unwrap()
                .value()
                .attr("class");
            let race_type = match selected {
                Some("green") => "特別",
                Some("yellow") => "準重賞",
                Some("pink") => "重賞",
                _ => "一般",
            };

            let date = NaiveDate::parse_from_str(&scrapped_row[0], "%Y/%m/%d").unwrap();
            let racecourse = Racecourse::from_str(&scrapped_row[1]);
            let race_num: i32 = scrapped_row[2].parse().unwrap();
            let horse_num: i32 = scrapped_row[11].parse().unwrap();
            let race = Race {
                date: date,
                racecourse: racecourse,
                race_num: race_num,
            };
            let racehorse = RaceHorse {
                date: date,
                racecourse: racecourse,
                race_num: race_num,
                horse_num: horse_num,
            };
            let horse_hisotry_dataracecourse = DateRacecourses {
                date_racecourse_id: DateRacecourse::new(date, racecourse).to_date_racecourse_id(),
                race_date: date.to_string(),
                racecourse: racecourse.to_japanese(),
                ..Default::default()
            };

            let horse_history_race = Races {
                race_id: race.to_race_id(),
                date_racecourse_id: DateRacecourse::new(date, racecourse).to_date_racecourse_id(),
                race_num: race_num,
                race_type: Some(race_type.to_string()),
                race_name: Some(scrapped_row[3].clone()).filter(|s| !s.is_empty()),
                surface: detect_surface(&scrapped_row[5]),
                distance: detect_num(&scrapped_row[5]),
                weather: Some(scrapped_row[6].clone()).filter(|s| !s.is_empty()),
                going: detect_going(&scrapped_row[7]),
                moisture: detect_num(&scrapped_row[7]),
                horse_count: Some(scrapped_row[9].clone()).filter(|s| !s.is_empty()),
                ..Default::default()
            };

            let horse_history_racehorse = RaceHorses {
                race_horse_id: racehorse.to_racehorse_id(),
                race_id: race.to_race_id(),
                bracket_num: Some(scrapped_row[10].clone())
                    .filter(|s| !s.is_empty())
                    .and_then(|f| f.parse().ok()),
                horse_num: scrapped_row[11].parse().unwrap(),
                win_fav: Some(scrapped_row[12].clone())
                    .filter(|s| !s.is_empty())
                    .and_then(|f| f.parse().ok()),
                arrival: Some(scrapped_row[13].clone()).filter(|s| !s.is_empty()),
                finish_time: convert_time(&scrapped_row[14]),
                margin_time: Some(scrapped_row[15].clone())
                    .filter(|s| !s.is_empty())
                    .and_then(|f| f.parse().ok()),
                last_3f: Some(scrapped_row[16].clone()).filter(|s| !s.is_empty()),
                horse_weight: Some(scrapped_row[17].clone())
                    .filter(|s| !s.is_empty())
                    .and_then(|f| f.parse().ok()),
                jockey_name: detect_before_bracket(&scrapped_row[18]),
                weight_to_carry: Some(scrapped_row[19].clone())
                    .filter(|s| !s.is_empty())
                    .and_then(|f| f.parse().ok()),
                trainer_name: Some(scrapped_row[20].clone()).filter(|s| !s.is_empty()),
                prize: Some(scrapped_row[21].clone())
                    .filter(|s| !s.is_empty())
                    .and_then(|f| f.replace(",", "").parse().ok()),
                horse_nar_id: Some(self.0.get_horse_id()),
                horse_name: Some(horse_name.clone()).filter(|s| !s.is_empty()),
                ..Default::default()
            };
            data.push(DbType::HorseHistoryBody(
                horse_hisotry_dataracecourse,
                horse_history_race,
                horse_history_racehorse,
            ));
        }
        Ok(data)
    }
}