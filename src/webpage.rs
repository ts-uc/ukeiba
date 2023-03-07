use anyhow::{anyhow, Result};
use regex::Regex;
use scraper::{Html, Selector};
use std::path::PathBuf;
pub mod bajikyo_search;
pub mod horse_history;
pub mod horse_profile;
pub mod oddspark_odds;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
pub mod race;
pub mod racelist;
pub mod rakuten_racelist;
use crate::db_writer::DbType;
use xz2::write::{XzDecoder, XzEncoder};

pub trait WebPageTrait {
    fn get_path(&self) -> PathBuf;
    fn fetch_string(&self) -> Result<String>;
    fn scrap(&self, body: &str) -> Result<Vec<DbType>>;
    fn exists(&self) -> bool {
        self.get_path().exists()
    }
    fn load_string(&self) -> Result<String> {
        let mut file = File::open(self.get_path()).map_err(|e| anyhow!(e))?;
        let mut file_buf = Vec::new();
        file.read_to_end(&mut file_buf).map_err(|e| anyhow!(e))?;

        let mut text_buf = Vec::new();
        let mut decoder = XzDecoder::new(text_buf);
        decoder.write_all(&file_buf).map_err(|e| anyhow!(e))?;
        text_buf = decoder.finish().map_err(|e| anyhow!(e))?;
        let text = String::from_utf8(text_buf).map_err(|e| anyhow!(e))?;

        Ok(text)
    }
    fn fetch(self) -> Result<WebPage<Self>>
    where
        Self: Sized,
    {
        let body = self.fetch_string()?;
        Ok(WebPage {
            web_page_trait: self,
            body: body,
        })
    }
    fn load(self) -> Result<WebPage<Self>>
    where
        Self: Sized,
    {
        let body = self.load_string()?;
        Ok(WebPage {
            web_page_trait: self,
            body: body,
        })
    }
    fn check_and_fetch(self, force_fetch: bool) -> Result<()>
    where
        Self: Sized,
    {
        if self.exists() && !force_fetch {
            return Ok(());
        }
        self.fetch()?.save()?;
        Ok(())
    }
}

pub struct WebPage<T> {
    web_page_trait: T,
    body: String,
}

impl<T: WebPageTrait> WebPage<T> {
    pub fn db(&self) -> Result<Vec<DbType>> {
        self.web_page_trait.scrap(&self.body)
    }
    pub fn save(&self) -> Result<()> {
        fs::create_dir_all(
            self.web_page_trait
                .get_path()
                .parent()
                .ok_or(anyhow!("get path parent error"))?,
        )
        .map_err(|e| anyhow!(e))?;

        let text_buf = self.body.as_bytes();
        let mut encoded_buf = XzEncoder::new(Vec::new(), 9);
        encoded_buf.write_all(&text_buf).map_err(|e| anyhow!(e))?;

        let buffer = encoded_buf.finish().map_err(|e| anyhow!(e))?;
        let mut file = File::create(self.web_page_trait.get_path()).map_err(|e| anyhow!(e))?;
        file.write_all(&buffer).map_err(|e| anyhow!(e))?;
        Ok(())
    }
}

fn get_from_url(url: &str) -> Result<String> {
    std::thread::sleep(std::time::Duration::from_millis(2000));
    log::info!("fetching {}", url);
    let res = reqwest::blocking::get(url)
        .map_err(|e| anyhow!(e))?
        .error_for_status()
        .map_err(|e| anyhow!(e))?;
    log::info!("Response: {:?} {}", &res.version(), &res.status());
    let text = res.text().map_err(|e| anyhow!(e))?;
    Ok(text)
}

fn scrap_grid(
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
                .to_string();
            low_scrapped.push(text);
        }
        scrapped.push(low_scrapped);
    }
    scrapped
}

fn scrap_text(document: &Html, selector: &Selector) -> Option<String> {
    Some(
        document
            .select(selector)
            .next()?
            .text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string(),
    )
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

fn detect_num<T: std::str::FromStr>(str: &str) -> Option<T> {
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

fn convert_time(str_time: &str) -> Option<f64> {
    let captured = regex::Regex::new(r"((\d+):)?([\d\.]+)")
        .unwrap()
        .captures(str_time)?;

    let minutes: f64 = captured.get(2).map_or("0", |m| m.as_str()).parse().ok()?;
    let seconds: f64 = captured.get(3).map_or("0", |m| m.as_str()).parse().ok()?;
    let time = minutes * 60.0 + seconds;
    Some(time)
}

fn get_req_param_num<T: std::str::FromStr>(element_ref: &Html, selector: &Selector) -> Option<T> {
    let id_url = element_ref
        .select(selector)
        .next()?
        .value()
        .attr("href")?
        .trim();
    let id_url = reqwest::Url::parse(&format!("http://example.com/{}", &id_url)).ok()?;
    let mut id_pairs = id_url.query_pairs();
    let (_, id) = id_pairs.next()?;
    let id = id.parse::<T>().ok()?;
    Some(id)
}

fn detect_before_bracket(str: &str) -> Option<String> {
    Some(Regex::new(r"\s*(.*?)\s*[\(（]").unwrap().captures(str)?[1].to_string())
        .filter(|s| !s.is_empty())
}

fn detect_inner_bracket(str: &str) -> Option<String> {
    Some(
        Regex::new(r"[\(（]\s*(.*?)\s*[\)）]")
            .unwrap()
            .captures(str)?[1]
            .to_string(),
    )
    .filter(|s| !s.is_empty())
}

fn detect_after_bracket(str: &str) -> Option<String> {
    Some(
        Regex::new(r"([\(（].*[\)）])?\s*(.*)")
            .unwrap()
            .captures(str)?[2]
            .to_string(),
    )
    .filter(|s| !s.is_empty())
}

fn remove_whitespace(str: &str) -> String {
    str.split_whitespace().collect::<Vec<_>>().join("")
}

fn scrap(html: &Html, selector_str: &str) -> Option<String> {
    let selector = Selector::parse(&selector_str).ok()?;
    let text = scrap_text(&html, &selector);
    text.filter(|s| !s.is_empty())
}

// fn scrap_param<T: std::str::FromStr>(html: &Html, selector_str: &str) -> Option<T> {
//     let selector = Selector::parse(&selector_str).unwrap();
//     get_req_param_num(html, &selector)
// }
