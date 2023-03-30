use anyhow::{anyhow, Result};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Duration;
use xz2::write::{XzDecoder, XzEncoder};

pub trait WebPageTrait {
    fn get_path(&self) -> PathBuf;

    fn fetch_string(&self, interval: Duration) -> Result<String>;

    fn exists(&self) -> bool {
        self.get_path().exists()
    }

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

    fn fetch(&self, interval: Duration) -> Result<WebPage<Self>>
    where
        Self: Clone,
    {
        let body = self.fetch_string(interval)?;
        Ok(WebPage {
            web_page_trait: self.clone(),
            body: body,
        })
    }

    fn load(&self) -> Result<WebPage<Self>>
    where
        Self: Clone,
    {
        let body = self.load_string()?;
        Ok(WebPage {
            web_page_trait: self.clone(),
            body: body,
        })
    }
}

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
