use crate::common::race::Race;
use crate::db_writer::RaceData;
use scraper::{Html, Selector};
use crate::db_writer::DbType;
use unicode_normalization::UnicodeNormalization;
use url::Url;

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
            let (bracket_num, bracket_num_index) = calc_wakuban(horse_count, horse_num);

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
            let jockey_name = text_getter(&document, &selector).split("(")
            .collect::<Vec<&str>>()[0]
            .to_string();
            let jockey_id: Option<i64> = get_req_param_num(&document, &selector);

            let selector_str = format!(".cardTable > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(2) > a:nth-child(1)", horse_num*5);
            let selector = Selector::parse(&selector_str).unwrap();
            let trainer_name = text_getter(&document, &selector).split("(")
            .collect::<Vec<&str>>()[0]
            .to_string();
            let trainer_id: Option<i64> = get_req_param_num(&document, &selector);

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
            let (sex, age) = detect_sexage(&sexage);
    
            let foo = RaceData{
                racehorse_id: self.race.to_race_horse(horse_num).to_racehorse_id(),
                race_id: self.race.to_race_id(),
                horse_num: horse_num,
                bracket_num: bracket_num,
                horse_name: horse_name,
                horse_sex: sex,
                horse_age: age,
                horse_id: horse_id,
                jockey_name: jockey_name,
                jockey_id: jockey_id,
                trainer_name: trainer_name,
                trainer_id: trainer_id,
                change: change,
                owner_name: owner_name,
                weight_mark: weight_mark,
                weight_to_carry: weight,
                horse_weight: horse_weight,
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

fn detect_sexage(course: &str) -> (String, i32) {
    let sex = if course.contains("牡") || course.contains("雄") {
        "牡".to_string()
    } else if course.contains("牝") || course.contains("雌") {
        "牝".to_string()
    }else if course.contains("セン") || course.contains("セ") {
        "セン".to_string()
    } else {"".to_string()};

    let age = course
        .replace("牡", "")
        .replace("雄", "")
        .replace("牝", "")
        .replace("雌", "")
        .replace("セン", "")
        .replace("セ", "")
        .parse()
        .unwrap();

    (sex, age)
}

