use scraper::{Html, Selector};
use unicode_normalization::UnicodeNormalization;


use crate::db_writer::Db;

pub mod racelist;
pub mod race;
pub mod horse_history;
pub mod horse_profile;

pub trait PageScraper{
    fn db(&self) -> Db;

}

fn grid_scrapper(document: &Html, row_selector: &Selector, column_selector: &Selector) -> Vec<Vec<String>>{
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

fn detect_going(raw_going: &str) -> (Option<String>, Option<f64>) {
    if raw_going == "" {
        return (None, None);
    };

    let going = match raw_going {
        "良" => Some("良".to_string()),
        "稍重" => Some("稍重".to_string()),
        "重" => Some("重".to_string()),
        "不良" => Some("不良".to_string()),
        _ => None,
    };

    let moisture: Option<f64> = raw_going.parse().ok();

    (going, moisture)
}

fn detect_corse(course: &str) -> (Option<String>, Option<String>, Option<i32>) {
    if course == "" {
        return (None, None, None);
    };
    let surface = if course.contains("芝") {
        Some("芝".to_string())
    } else {
        Some("ダ".to_string())
    };

    let direction = if course.contains("右") {
        Some("右".to_string())
    } else if course.contains("左") {
        Some("左".to_string())
    } else if course.contains("直") {
        Some("直".to_string())
    } else {
        None
    };

    let distance = course
        .replace("芝", "")
        .replace("右", "")
        .replace("左", "")
        .replace("直", "")
        .replace("m", "")
        .parse()
        .ok();

    (surface, direction, distance)
}

fn detect_sexage(course: &str) -> (String, i32) {
    let sex = if course.contains("牡") || course.contains("雄") {
        "牡".to_string()
    } else if course.contains("牝") || course.contains("雌") {
        "牝".to_string()
    } else if course.contains("セン") || course.contains("セ") {
        "セン".to_string()
    } else {
        "".to_string()
    };

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
