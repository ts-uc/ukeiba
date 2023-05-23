use anyhow::{anyhow, Result};
use scraper::{ElementRef, Selector};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Duration;
use xz2::write::{XzDecoder, XzEncoder};
pub mod bajikyo_search;
pub mod horse_history;
pub mod horse_profile;
pub mod horse_search;
pub mod jockey_profile;
pub mod jockey_search;
pub mod trainer_profile;
pub mod trainer_search;

#[derive(Debug, Clone)]
pub enum Mode {
    Fetch,
    Load,
    Normal,
    FetchSave,
    NormalSave,
}

pub trait WebPageTrait {
    type Data;

    fn get_path(&self) -> PathBuf;

    fn fetch_string(&self, interval: Duration) -> Result<String>;

    fn scrap_string(&self, body: &str) -> Result<Self::Data>;

    fn load_string(&self) -> Result<String> {
        let mut file = File::open(self.get_path())?;
        let mut file_buf = Vec::new();
        file.read_to_end(&mut file_buf)?;

        let mut text_buf = Vec::new();
        let mut decoder = XzDecoder::new(text_buf);
        decoder.write_all(&file_buf)?;
        text_buf = decoder.finish()?;
        let text = String::from_utf8(text_buf)?;

        Ok(text)
    }

    fn save_string(&self, body: &str) -> Result<()> {
        create_dir_all(
            self.get_path()
                .parent()
                .ok_or(anyhow!("get path parent error"))?,
        )?;

        let text_buf = body.as_bytes();
        let mut encoded_buf = XzEncoder::new(Vec::new(), 9);
        encoded_buf.write_all(&text_buf)?;

        let buffer = encoded_buf.finish()?;
        let mut file = File::create(self.get_path())?;
        file.write_all(&buffer)?;

        Ok(())
    }

    fn fetch_scrap(&self, mode: Mode, interval: Duration) -> Result<Self::Data>
    where
        Self: Clone,
    {
        let (text, is_save) = match mode {
            Mode::Load => (self.load_string()?, false),
            Mode::FetchSave => (self.fetch_string(interval)?, true),
            Mode::Fetch => (self.fetch_string(interval)?, false),
            Mode::NormalSave => {
                if self.get_path().exists() {
                    (self.load_string()?, false)
                } else {
                    (self.fetch_string(interval)?, true)
                }
            }
            Mode::Normal => {
                if self.get_path().exists() {
                    (self.load_string()?, false)
                } else {
                    (self.fetch_string(interval)?, false)
                }
            }
        };
        if is_save {
            self.save_string(&text)?;
        }
        self.scrap_string(&text)
    }

    fn fetch(&self, interval: Duration) -> Result<()> {
        if self.get_path().exists() {
            return Ok(());
        }
        let text = self.fetch_string(interval)?;
        self.save_string(&text)?;
        Ok(())
    }

    fn force_fetch(&self, interval: Duration) -> Result<()> {
        let text = self.fetch_string(interval)?;
        self.save_string(&text)?;
        Ok(())
    }

    fn scrap(&self) -> Result<Self::Data> {
        let text = self.load_string()?;
        self.scrap_string(&text)
    }
}

fn get_from_url(url: &str, interval: Duration) -> Result<String> {
    std::thread::sleep(interval);
    let res = reqwest::blocking::get(url)?.error_for_status()?;
    let text = res.text()?;
    Ok(text)
}

fn scrap(html: &ElementRef, selector_str: &str) -> Option<String> {
    let selector = Selector::parse(&selector_str).ok()?;
    html.select(&selector)
        .next()
        .map(|x| x.inner_html().trim().to_string())
        .filter(|s| !s.is_empty())
}

fn scrap_remove_tag(html: &ElementRef, selector_str: &str) -> Option<String> {
    let selector = Selector::parse(&selector_str).ok()?;
    html.select(&selector)
        .next()
        .map(|x| x.text().collect::<Vec<_>>().join("").trim().to_string())
        .filter(|s| !s.is_empty())
}

fn scrap_link(html: &ElementRef, selector_str: &str) -> Option<String> {
    let selector = Selector::parse(&selector_str).ok()?;
    html.select(&selector)
        .next()?
        .value()
        .attr("href")
        .map(str::to_string)
}

fn remove_whitespace(str: &str) -> String {
    str.split_whitespace().collect::<Vec<_>>().join("")
}

fn split_bracket<'a>(raw: &'a str) -> (&'a str, &'a str, &'a str) {
    let captured = regex::Regex::new(r"^\s*(.*?)\s*\(\s*(.*?)\s*\)\s*(.*?)\s*$")
        .unwrap()
        .captures(&raw);
    match captured {
        Some(x) => (
            x.get(1).map_or("", |x| x.as_str()),
            x.get(2).map_or("", |x| x.as_str()),
            x.get(3).map_or("", |x| x.as_str()),
        ),
        None => (raw, "", raw),
    }
}

fn convert_time(str_time: &str) -> Option<f64> {
    let captured = regex::Regex::new(r"((\d+):)?([\d\.]+)")
        .unwrap()
        .captures(str_time)?;

    let minutes: f64 = captured.get(2).map_or("0", |m| m.as_str()).parse().ok()?;
    let seconds: f64 = captured.get(3).map_or("0", |m| m.as_str()).parse().ok()?;
    let time = minutes * 60.0 + seconds;
    Some(time)
}

fn get_query<'a>(url: &'a str, query: &str) -> Option<&'a str> {
    let regex_str = format!("[?&]{}=(.*?)(&|$)", query);
    let r = regex::Regex::new(&regex_str)
        .ok()?
        .captures(&url)?
        .get(1)?
        .as_str();
    Some(r)
}
