use crate::common::horse::Horse;
use crate::common::race::Race;
use crate::common::race_horse::RaceHorse;
use crate::common::racecourse::Racecourse;
use crate::db_writer::DbType;
use crate::db_writer::RaceHorses;
use crate::db_writer::Races;
use crate::webpage::{detect_corse, detect_going, grid_scrapper};
use chrono::NaiveDate;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct PageHorseHistory {
    pub html: String,
    pub horse: Horse,
}

impl PageHorseHistory {
    pub fn new(html: String, horse: Horse) -> Self {
        Self {
            html: html,
            horse: horse,
        }
    }

    pub fn db(&self) -> Vec<DbType> {
        let document = Html::parse_document(&self.html);
        let row_selector = ".HorseMarkInfo_table > tbody:nth-child(2) > tr";
        let row_selector = Selector::parse(row_selector).unwrap();
        let column_selector = "td";
        let column_selector = Selector::parse(column_selector).unwrap();
        let horse_name_selector = ".odd_title";
        let horse_name_selector = Selector::parse(horse_name_selector).unwrap();

        let scrapped = grid_scrapper(&document, &row_selector, &column_selector);
        let horse_name = document
            .select(&horse_name_selector)
            .next()
            .unwrap()
            .inner_html();

        let mut data = Vec::new();

        for scrapped_row in scrapped {
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
            let (surface, _, distance) = detect_corse(&scrapped_row[5]);
            let (going, moisture) = detect_going(&scrapped_row[7]);
            let horse_history_race = Races {
                race_id: race.to_race_id(),
                race_date: date.to_string(),
                racecourse: racecourse.to_string(),
                race_num: race_num,
                change: None,
                race_type: None,
                race_name: Some(scrapped_row[3].clone()).filter(|s| !s.is_empty()),
                surface: surface,
                distance: distance,
                weather: Some(scrapped_row[6].clone()).filter(|s| !s.is_empty()),
                going: going,
                moisture: moisture,
                horse_count: Some(scrapped_row[9].clone()).filter(|s| !s.is_empty()),
                post_time: None,
                direction: None,
            };

            data.push(DbType::HorseHistoryRace(horse_history_race));

            let horse_history_racehorse = RaceHorses {
                race_horse_id: racehorse.to_racehorse_id(),
                race_id: race.to_race_id(),
                bracket_num: Some(scrapped_row[10].clone()).filter(|s| !s.is_empty()),
                horse_num: scrapped_row[11].parse().unwrap(),
                win_fav: Some(scrapped_row[12].clone()).filter(|s| !s.is_empty()),
                arrival: Some(scrapped_row[13].clone()).filter(|s| !s.is_empty()),
                finish_time: Some(scrapped_row[14].clone()).filter(|s| !s.is_empty()),
                margin_time: Some(scrapped_row[15].clone()).filter(|s| !s.is_empty()),
                last_3f: Some(scrapped_row[16].clone()).filter(|s| !s.is_empty()),
                horse_weight: Some(scrapped_row[17].clone()).filter(|s| !s.is_empty()),
                jockey_name: Some(
                    scrapped_row[18]
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .join(""),
                )
                .filter(|s| !s.is_empty()),
                weight_to_carry: Some(scrapped_row[19].clone()).filter(|s| !s.is_empty()),
                trainer_name: Some(scrapped_row[20].clone()).filter(|s| !s.is_empty()),
                prize: Some(scrapped_row[21].clone()).filter(|s| !s.is_empty()),
                horse_id: Some(self.horse.get_horse_id()),
                horse_age: None,
                horse_name: Some(horse_name.clone()).filter(|s| !s.is_empty()),
                horse_sex: None,
                horse_weight_delta: None,
                jockey_id: None,
                trainer_id: None,
                change: None,
                owner_name: None,
                weight_mark: None,
                margin: None,
                win_odds: None,
                place_odds_max: None,
                place_odds_min: None,
            };
            data.push(DbType::HorseHistoryRaceHorse(horse_history_racehorse));
        }
        data
    }
}
