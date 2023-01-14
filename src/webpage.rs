use regex::Regex;
use scraper::{Html, Selector};
use unicode_normalization::UnicodeNormalization;

use crate::db_writer::Db;

pub mod horse_history;
pub mod horse_profile;
pub mod race;
pub mod racelist;

pub trait PageScraper {
    fn db(&self) -> Db;
}

fn grid_scrapper(
    document: &Html,
    row_selector: &Selector,
    column_selector: &Selector,
) -> Vec<Vec<String>> {
    let mut scrapped: Vec<Vec<String>> = Vec::new();
    let low_selected = document.select(&row_selector);
    for low_ref in low_selected {
        let mut low_scrapped: Vec<String> = Vec::new();
        let column_selected = low_ref.select(&column_selector);
        for column_ref in column_selected {
            let text = column_ref
                .text()
                .collect::<Vec<_>>()
                .join("")
                .trim()
                .nfkc()
                .collect();
            low_scrapped.push(text);
        }
        scrapped.push(low_scrapped);
    }
    scrapped
}

fn detect_going(str: &str) -> Option<String> {
    Some(
        Regex::new(r"(良|稍重|重|不良)")
            .unwrap()
            .find(str)?
            .as_str()
            .to_string(),
    )
}

fn detect_surface(str: &str) -> Option<String> {
    if str.contains("芝") {
        Some("芝".to_string())
    } else {
        None
    }
}

fn detect_direction(str: &str) -> Option<String> {
    Some(
        Regex::new(r"(右|左|直)")
            .unwrap()
            .find(str)?
            .as_str()
            .to_string(),
    )
}

fn detect_int<T: std::str::FromStr>(str: &str) -> Option<T> {
    Some(
        Regex::new(r"\d+(\.\d+)?")
            .unwrap()
            .find(str)?
            .as_str()
            .parse()
            .ok()?,
    )
}

fn detect_horse_sex(course: &str) -> Option<String> {
    if course.contains("牡") || course.contains("雄") {
        Some("牡".to_string())
    } else if course.contains("牝") || course.contains("雌") {
        Some("牝".to_string())
    } else if course.contains("セ") || course.contains("騙") {
        Some("セン".to_string())
    } else {
        None
    }
}
