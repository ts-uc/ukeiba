use anyhow::{anyhow, Result};
use scraper::{ElementRef, Selector};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Duration;
use xz2::write::{XzDecoder, XzEncoder};
pub mod horse_history;
pub mod horse_profile;

#[derive(Debug, Clone)]
pub enum Mode {
    Fetch,
    Load,
    Normal,
    FetchSave,
    NormalSave,
}

pub trait WebPageTrait {
    fn get_path(&self) -> PathBuf;

    fn fetch_string(&self, interval: Duration) -> Result<String>;

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

    fn get(&self, mode: Mode, interval: Duration) -> Result<WebPage<Self>>
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
        let webpage = WebPage {
            web_page_trait: self.clone(),
            body: text,
        };
        if is_save {
            webpage.save()?;
        }
        Ok(webpage)
    }
}

#[derive(Debug, Clone)]
pub struct WebPage<T> {
    web_page_trait: T,
    body: String,
}

impl<T: WebPageTrait> WebPage<T> {
    pub fn save(&self) -> Result<()> {
        create_dir_all(
            self.web_page_trait
                .get_path()
                .parent()
                .ok_or(anyhow!("get path parent error"))?,
        )?;

        let text_buf = self.body.as_bytes();
        let mut encoded_buf = XzEncoder::new(Vec::new(), 9);
        encoded_buf.write_all(&text_buf)?;

        let buffer = encoded_buf.finish()?;
        let mut file = File::create(self.web_page_trait.get_path())?;
        file.write_all(&buffer)?;
        Ok(())
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

fn split_blacket<'a>(raw: &'a str) -> (&'a str, &'a str, &'a str) {
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
