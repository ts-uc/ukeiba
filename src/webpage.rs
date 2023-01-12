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

fn grid_scrapper(document: Html, row_selector: Selector, column_selector: Selector) -> Vec<Vec<String>>{
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
