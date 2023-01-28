use super::*;
use crate::common::date_racecourse::DateRacecourse;
use crate::common::race::Race;
use crate::db_writer::Races;
use crate::db_writer::DbType;
use crate::db_writer::RaceHorses;
use scraper::{Html, Selector};
use unicode_normalization::UnicodeNormalization;

#[derive(Debug)]
pub struct PageRace {
    pub html: String,
    pub race: Race,
}

impl PageRace {
    pub fn new(html: String, race: Race) -> Self {
        Self {
            html: html,
            race: race,
        }
    }

    pub fn db(&self) -> Vec<DbType> {
        let document: String = self.html.nfkc().collect();
        let document = Html::parse_document(&document);

        let race_data = scrap(&document, "ul.dataArea:nth-child(5) > li:nth-child(1)");
        let (race_horse_type, race_age, race_weight_type) = race_data_split(race_data);

        let selector_str = ".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr";
        let selector = Selector::parse(selector_str).unwrap();

        let horse_count = ((document.select(&selector).count() - 2) / 5) as i32;

        let mut data: Vec<DbType> = Vec::new();

        let title_data = Races{
            race_id: self.race.to_race_id(),
            date_racecourse_id: DateRacecourse::new(self.race.date,self.race.racecourse).to_date_racecourse_id(),
            race_num: self.race.race_num,
            race_name: scrap(&document, ".raceTitle > h3:nth-child(4)"),
            race_sub_title: scrap(&document, ".subTitle"),
            race_horse_type: race_horse_type,
            race_age: race_age,
            race_weight_type: race_weight_type,
            post_time: None,
            change: None,
            race_type: None,
            surface: None,
            direction: None,
            distance: None,
            weather: None,
            going: None,
            moisture: None,
            horse_count: None
        };

        data.push(DbType::RaceRaces(title_data));

        for horse_num in 1..=horse_count {
            let (_, bracket_num_index) = calc_wakuban(horse_count, horse_num);

            let selector_str = format!(
                "tr.tBorder:nth-child({}) > td:nth-child({}) > a:nth-child(1)",
                horse_num * 5 - 2,
                3 - bracket_num_index
            );
            let selector = Selector::parse(&selector_str).unwrap();
            let horse_name = scrap_text(&document, &selector);
            let horse_id: Option<i64> = get_req_param_num(&document, &selector);

            let selector_str = format!(
                "tr.tBorder:nth-child({}) > td:nth-child({}) > a:nth-child(1)",
                horse_num * 5 - 2,
                4 - bracket_num_index
            );
            let selector = Selector::parse(&selector_str).unwrap();
            let jockey = scrap_text(&document, &selector);
            let jockey_name = detect_before_bracket(&jockey.unwrap());
            let jockey_id: Option<i32> = get_req_param_num(&document, &selector);

            let selector_str = format!(".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(2) > a:nth-child(1)", horse_num*5);
            let selector = Selector::parse(&selector_str).unwrap();
            let trainer = scrap_text(&document, &selector);
            let trainer_name = detect_before_bracket(&trainer.unwrap());
            let trainer_id: Option<i32> = get_req_param_num(&document, &selector);

            let owner_name = scrap(&document, &format!(".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(2)", horse_num*5+1));

            let weight = scrap(&document, &format!(".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(4)", horse_num*5-1)).unwrap();
            let weight_arr = weight.split_whitespace().collect::<Vec<&str>>();
            let weight = if weight_arr.len() == 3 {
                weight_arr[1].to_string()
            } else {
                weight_arr[0].to_string()
            };
            let weight_mark = if weight_arr.len() == 3 {
                weight_arr[0].to_string()
            } else {
                "".to_string()
            };

            let change = scrap(&document, &format!(".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(3)", horse_num*5+2));

            let horse_weight_and_delta = scrap(&document, &format!(".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(3)", horse_num*5));
            let horse_weight = detect_before_bracket(&horse_weight_and_delta.unwrap());

            let sexage = scrap(&document, &format!(".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(1)", horse_num*5-1));
            let sex = detect_horse_sex(&sexage.unwrap());

            let foo = RaceHorses {
                race_horse_id: self.race.to_race_horse(horse_num).to_racehorse_id(),
                race_id: self.race.to_race_id(),
                horse_num: horse_num,
                bracket_num: None,
                horse_name: horse_name.filter(|s| !s.is_empty()),
                horse_sex: sex,
                horse_age: None,
                horse_id: horse_id,
                jockey_name: jockey_name,
                jockey_id: jockey_id,
                trainer_name: trainer_name,
                trainer_id: trainer_id,
                change: change.filter(|s| !s.is_empty()),
                owner_name: owner_name.filter(|s| !s.is_empty()),
                weight_mark: Some(weight_mark).filter(|s| !s.is_empty()),
                weight_to_carry: Some(weight).filter(|s| !s.is_empty()),
                horse_weight: horse_weight,
                horse_weight_delta: None,
                arrival: None,
                finish_time: None,
                margin_time: None,
                margin: None,
                last_3f: None,
                win_fav: None,
                win_odds: None,
                place_odds_min: None,
                place_odds_max: None,
                prize: None,
            };

            data.push(DbType::Race(foo));
        }
        data
    }
}

fn calc_wakuban(horse_count: i32, horse_num: i32) -> (i32, i32) {
    if horse_count <= 8 {
        (horse_num, 0)
    } else {
        let base_num = 16 - horse_count;
        if horse_num <= base_num {
            (horse_num, 0)
        } else {
            let foo = horse_num - base_num + 1;
            (base_num + foo / 2, foo % 2)
        }
    }
}

fn race_data_split(data: Option<String>) -> (Option<String>, Option<String>, Option<String>) {
    match data {
        Some(x) => {
            let a: Vec<&str> = x.split_whitespace().collect();
            if a.len() < 4{
                return (None, None, None)
            }
            let b = a[a.len() - 4].to_string();
            let c = a[a.len() - 3].to_string();
            let d = a[a.len() - 2].to_string();
            (Some(b), Some(c), Some(d))
        }
        None => (None, None, None),
    }
}
