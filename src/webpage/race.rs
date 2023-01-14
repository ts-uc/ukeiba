use crate::common::race::Race;
use crate::db_writer::DbType;
use crate::db_writer::RaceHorses;
use scraper::{Html, Selector};
use unicode_normalization::UnicodeNormalization;
use url::Url;
use crate::webpage::detect_sexage;

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
        let document = Html::parse_document(&self.html);

        let selector_str = ".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr";
        let selector = Selector::parse(selector_str).unwrap();

        let horse_count = ((document.select(&selector).count() - 2) / 5) as i32;

        let mut race_horse_list: Vec<DbType> = Vec::new();
        for horse_num in 1..=horse_count {
            let (_, bracket_num_index) = calc_wakuban(horse_count, horse_num);

            let selector_str = format!(
                "tr.tBorder:nth-child({}) > td:nth-child({}) > a:nth-child(1)",
                horse_num * 5 - 2,
                3 - bracket_num_index
            );
            let selector = Selector::parse(&selector_str).unwrap();
            let horse_name = text_getter(&document, &selector);
            let horse_id: Option<i64> = get_req_param_num(&document, &selector);

            let selector_str = format!(
                "tr.tBorder:nth-child({}) > td:nth-child({}) > a:nth-child(1)",
                horse_num * 5 - 2,
                4 - bracket_num_index
            );
            let selector = Selector::parse(&selector_str).unwrap();
            let jockey_name = text_getter(&document, &selector)
                .split("(")
                .collect::<Vec<&str>>()[0]
                .to_string();
            let jockey_id: Option<i32> = get_req_param_num(&document, &selector);

            let selector_str = format!(".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(2) > a:nth-child(1)", horse_num*5);
            let selector = Selector::parse(&selector_str).unwrap();
            let trainer_name = text_getter(&document, &selector)
                .split("(")
                .collect::<Vec<&str>>()[0]
                .to_string();
            let trainer_id: Option<i32> = get_req_param_num(&document, &selector);

            let selector_str = format!(".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(2)", horse_num*5+1);
            let selector = Selector::parse(&selector_str).unwrap();
            let owner_name = text_getter(&document, &selector);

            let selector_str = format!(".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(4)", horse_num*5-1);
            let selector = Selector::parse(&selector_str).unwrap();
            let weight = text_getter(&document, &selector);
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

            let selector_str = format!(".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(3)", horse_num*5+2);
            let selector = Selector::parse(&selector_str).unwrap();
            let change = text_getter(&document, &selector);

            let selector_str = format!(".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(3)", horse_num*5);
            let selector = Selector::parse(&selector_str).unwrap();
            let horse_weight = text_getter(&document, &selector)
                .split("(")
                .collect::<Vec<&str>>()[0]
                .to_string();

            let selector_str = format!(".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(1)", horse_num*5-1);
            let selector = Selector::parse(&selector_str).unwrap();
            let sexage = text_getter(&document, &selector);
            let (sex, _) = detect_sexage(&sexage);

            let foo = RaceHorses {
                race_horse_id: self.race.to_race_horse(horse_num).to_racehorse_id(),
                race_id: self.race.to_race_id(),
                horse_num: horse_num,
                bracket_num: None,
                horse_name: Some(horse_name).filter(|s| !s.is_empty()),
                horse_sex: Some(sex).filter(|s| !s.is_empty()),
                horse_age: None,
                horse_id: horse_id,
                jockey_name: Some(jockey_name).filter(|s| !s.is_empty()),
                jockey_id: jockey_id,
                trainer_name: Some(trainer_name).filter(|s| !s.is_empty()),
                trainer_id: trainer_id,
                change: Some(change).filter(|s| !s.is_empty()),
                owner_name: Some(owner_name).filter(|s| !s.is_empty()),
                weight_mark: Some(weight_mark).filter(|s| !s.is_empty()),
                weight_to_carry: Some(weight).filter(|s| !s.is_empty()),
                horse_weight: Some(horse_weight).filter(|s| !s.is_empty()),
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

            race_horse_list.push(DbType::Race(foo));
        }
        race_horse_list
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

fn text_getter(element_ref: &Html, selector: &Selector) -> String {
    element_ref
        .select(selector)
        .next()
        .unwrap()
        .text()
        .collect::<Vec<_>>()
        .join("")
        .trim()
        .nfkc()
        .collect::<String>()
}

fn get_req_param_num<T: std::str::FromStr>(element_ref: &Html, selector: &Selector) -> Option<T> {
    let id_url = element_ref
        .select(selector)
        .next()?
        .value()
        .attr("href")?
        .trim();
    let id_url = Url::parse(&format!("http://example.com/{}", &id_url)).ok()?;
    let mut id_pairs = id_url.query_pairs();
    let (_, id) = id_pairs.next()?;
    let id = id.parse::<T>().ok()?;
    Some(id)
}
