use crate::common::*;
use chrono::{Date, Local};
use std::fs::{File, ReadDir};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::enums::Racecourse;

pub fn save_racelist(date: &Date<Local>, racecourse: &Racecourse, body: &str) {
    let b = body.as_bytes();
    let filename = dirs::data_dir()
        .unwrap()
        .join("ukeiba")
        .join("racelist")
        .join(format!(
            "racelist_{}.html",
            gen_racelistid(&date, &racecourse)
        ));
    let mut file = File::create(filename).unwrap();
    file.write_all(b).unwrap();
}

pub fn racelist_dir() -> ReadDir{
    let path = dirs::data_dir()
    .unwrap()
    .join("ukeiba")
    .join("racelist");
    fs::read_dir(path).unwrap()
}