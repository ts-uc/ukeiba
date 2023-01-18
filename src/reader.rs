use flate2::write::{GzDecoder, GzEncoder};
use flate2::Compression;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub mod racelist;
pub mod race;
pub mod horse_history;
pub mod horse_profile;
pub mod oddspark_odds;
pub mod rakuten_racelist;

// pub trait Reader<T> where T: super::WebPage,

pub trait Reader {
    //fn open(&self) -> T;
    fn get_file_path(&self) -> std::path::PathBuf;

    fn get_file_dir_path(&self) -> std::path::PathBuf;

    fn get_url(&self) -> String;

    fn fetch_string(&self) -> Option<String> {
        std::thread::sleep(std::time::Duration::from_millis(2000));
        let url = self.get_url();
        log::info!("fetching {}", url);
        let res = reqwest::blocking::get(url).ok()?;
        log::info!("Response: {:?} {}", &res.version(), &res.status());
        let text = res.text().ok()?;
        Some(text)
    }

    fn open_string(&self) -> Option<String> {
        let mut file = File::open(self.get_file_path()).ok()?;
        let mut file_buf = Vec::new();
        file.read_to_end(&mut file_buf).ok()?;

        let mut text_buf = Vec::new();
        let mut decoder = GzDecoder::new(text_buf);
        decoder.write_all(&file_buf).ok()?;
        text_buf = decoder.finish().ok()?;
        let text = String::from_utf8(text_buf).ok()?;

        Some(text)
    }

    fn save_string(&self, text: &str) {
        fs::create_dir_all(self.get_file_dir_path()).unwrap();

        let text_buf = text.as_bytes();
        let mut encoded_buf = GzEncoder::new(Vec::new(), Compression::default());
        encoded_buf.write_all(&text_buf).unwrap();

        let buffer = encoded_buf.finish().unwrap();
        let mut file = File::create(self.get_file_path()).unwrap();
        file.write_all(&buffer).unwrap();
    }

    fn get_string(&self, is_force_fetch: bool, is_save: bool) -> Option<String> {
        let text = match is_force_fetch {
            true => self.fetch_string()?,
            false => match self.open_string() {
                Some(text) => return Some(text),
                None => self.fetch_string()?,
            },
        };

        if is_save {
            self.save_string(&text)
        }
        Some(text)
    }

    fn get_save_string(&self, is_force_fetch: bool) -> Option<String> {
        self.get_string(is_force_fetch, true)
    }
}
