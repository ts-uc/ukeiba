use super::*;
use crate::common::race::Race;
use crate::db_writer::DbType;
use crate::db_writer::RaceHorses;
use scraper::{Html, Selector};
use unicode_normalization::UnicodeNormalization;

#[derive(Debug)]
pub struct PageOddsparkOdds {
    pub html: String,
    pub race: Race,
}

impl PageOddsparkOdds {
    pub fn new(html: String, race: Race) -> Self {
        Self {
            html: html,
            race: race,
        }
    }

    pub fn db(&self) -> Vec<DbType> {
        let data: Vec<DbType> = Vec::new();
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
