extern crate ukeiba_common;
use anyhow::Result;
use chrono::{Datelike, NaiveDate};
use csv::Writer;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use rayon::prelude::*;
use serde::Serialize;
use std::time::Duration;
use ukeiba_common::scraper::WebPageTrait;

pub fn get_fiscal_year(date: NaiveDate) -> Option<i32> {
    let fiscal_year_start = NaiveDate::from_ymd_opt(date.year(), 4, 1)?;

    let fiscal_year = if date < fiscal_year_start {
        date.year() - 1
    } else {
        date.year()
    };
    Some(fiscal_year)
}
pub fn fetch_and_scrap_all<T>(pages: Vec<T>) -> Vec<T::Data>
where
    T::Data: Send,
    T: WebPageTrait + Sync,
{
    fetch_all(&pages);
    scrap_all(pages)
}

pub fn fetch_all<T: WebPageTrait>(pages: &[T]) {
    pages
        .iter()
        .progress()
        .filter_map(|page| page.fetch(Duration::from_millis(700)).ok())
        .for_each(drop);
}

pub fn scrap_all<T>(pages: Vec<T>) -> Vec<T::Data>
where
    T::Data: Send,
    T: WebPageTrait + Sync,
{
    pages
        .par_iter()
        .progress_count(pages.len() as u64)
        .map(|page| page.scrap())
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

#[allow(dead_code)]
pub fn write_csv<T>(filename: &str, data: &[T]) -> Result<()>
where
    T: Serialize,
{
    let mut writer = Writer::from_path(filename)?;

    for record in data {
        writer.serialize(record)?;
    }

    writer.flush()?;
    Ok(())
}
