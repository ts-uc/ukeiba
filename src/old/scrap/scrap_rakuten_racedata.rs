#![allow(unused)]
//現在不使用。再使用予定

use crate::Race;
use crate::Racecourse;
use crate::Surface;
use crate::Going;
use crate::RaceData;

use chrono::prelude::*;
use scraper::Html;
use scraper::Selector;
use thiserror::Error;
use unicode_normalization::UnicodeNormalization;

type Result<T> = std::result::Result<T, CustomError>;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("data store disconnected")]
    SelectorParseError,
}

pub trait RaceDataTrait {
    fn scrap_str(&self, selectors: &str) -> Result<String>;
    fn scrap_th(&self) -> Option<i32>;
    fn scrap_weather(&self) -> Option<String>;
    fn scrap_day(&self) -> Option<i32>;
    fn scrap_distance(&self) -> (Option<Surface>, Option<i32>);
    fn scrap_going(&self) -> (Option<Going>, Option<f64>);
    fn scrap_post_time(&self) -> Option<String>;
    fn scrap_name(&self) -> Option<String>;
    fn scrap_horse_condition(&self) -> (Option<String>, Option<String>);
    fn detect_class(&self, race_name: &Option<&str>, racecourse: &Racecourse) -> Option<String>;
    fn scrap_race_data(&self, race: Race) -> RaceData;
}

impl RaceDataTrait for Html {
    fn scrap_str(&self, selectors: &str) -> Result<String> {
        let weatcher_selector: Selector = Selector::parse(selectors)
            .ok()
            .ok_or(CustomError::SelectorParseError)?;
        let r = self
            .select(&weatcher_selector)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()[0]
            .trim()
            .nfkc()
            .collect::<String>();
        Ok(r)
    }

    fn scrap_th(&self) -> Option<i32> {
        let scrapped = match self.scrap_str("ul.trackState:nth-child(1) > li:nth-child(2)") {
            Ok(scrapped) => scrapped,
            Err(_) => return None,
        };
        scrapped.replace("第", "").replace("回", "").parse().ok()
    }

    fn scrap_day(&self) -> Option<i32> {
        let scrapped = match self.scrap_str("ul.trackState:nth-child(1) > li:nth-child(4)") {
            Ok(scrapped) => scrapped,
            Err(_) => return None,
        };
        scrapped.replace("第", "").replace("日", "").parse().ok()
    }

    fn scrap_distance(&self) -> (Option<Surface>, Option<i32>) {
        let scrapped = match self.scrap_str(".distance") {
            Ok(scrapped) => scrapped,
            Err(_) => return (None, None),
        };

        let surface = if scrapped.contains("ダ") {
            Some(Surface::Dirt)
        } else if scrapped.contains("芝") {
            Some(Surface::Turf)
        } else {
            None
        };

        let distance: Option<i32> = scrapped
            .replace("ダ", "")
            .replace("芝", "")
            .replace("m", "")
            .parse()
            .ok();

        (surface, distance)
    }

    fn scrap_weather(&self) -> Option<String> {
        self.scrap_str(
            "ul.trackState:nth-child(2) > li:nth-child(2) > dl:nth-child(1) > dd:nth-child(2)",
        )
        .ok()
    }

    fn scrap_going(&self) -> (Option<Going>, Option<f64>) {
        let scrapped = match self.scrap_str(
            "ul.trackState:nth-child(2) > li:nth-child(2) > dl:nth-child(1) > dd:nth-child(4)",
        ) {
            Ok(scrapped) => scrapped,
            Err(_) => return (None, None),
        };

        let going = match &*scrapped {
            "良" => Some(Going::GoodToFirm),
            "稍重" => Some(Going::Good),
            "重" => Some(Going::Yielding),
            "不良" => Some(Going::Soft),
            _ => None,
        };

        let moisture: Option<f64> = scrapped.replace("%", "").parse().ok();

        (going, moisture)
    }

    fn scrap_post_time(&self) -> Option<String> {
        self.scrap_str(
            "ul.trackState:nth-child(2) > li:nth-child(2) > dl:nth-child(1) > dd:nth-child(6)",
        )
        .ok()
    }

    fn scrap_name(&self) -> Option<String> {
        self.scrap_str(".raceNote > h2:nth-child(3)").ok()
    }

    fn scrap_horse_condition(&self) -> (Option<String>, Option<String>) {
        let scrapped = match self.scrap_str(".horseCondition > li:nth-child(1)") {
            Ok(scrapped) => scrapped,
            Err(_) => return (None, None),
        };

        let scrapped_vec: Vec<&str> = scrapped.split_whitespace().collect();

        if scrapped_vec.len() != 2 {
            return (None, None);
        }
        return (
            Some(scrapped_vec[0].to_string()),
            Some(scrapped_vec[1].to_string()),
        );
    }

    fn detect_class(&self, race_name: &Option<&str>, racecourse: &Racecourse) -> Option<String> {
        let race_name = match race_name{
            Some(race_name) => race_name,
            None => return None,
        };

        if race_name.contains("オープン-") {
            Some("オープン".to_string())
        } else if race_name.contains("A1-") {
            Some("A1".to_string())
        } else if race_name.contains("A2-") {
            Some("A2".to_string())
        } else if race_name.contains("B1-") {
            Some("B1".to_string())
        } else if race_name.contains("B2-") {
            Some("B2".to_string())
        } else if race_name.contains("B3-") {
            Some("B3".to_string())
        } else if race_name.contains("B4-") {
            Some("B4".to_string())
        } else if race_name.contains("C1-") {
            Some("C1".to_string())
        } else if race_name.contains("C2-") {
            Some("C2".to_string())
        } else {
            None
        }
    }

    fn scrap_race_data(&self, race: Race) -> RaceData {
        let (surface, distance) = self.scrap_distance();
        let (going, moisture) = self.scrap_going();
        let race_name = self.scrap_name();
        let class = self.detect_class(&race_name.as_deref(), &race.racecourse);
        let (breed, age) = self.scrap_horse_condition();
        
        RaceData {
            date: race.date,
            racecourse: race.racecourse,
            race: race.num,
            th: self.scrap_th(),
            day: self.scrap_day(),
            surface: surface,
            distance: distance,
            weather: self.scrap_weather(),
            going: going,
            moisture: moisture,
            posttime: self.scrap_post_time(),
            name: race_name,
            class: class,
            breed: breed,
            age: age,
        }
    }
}
