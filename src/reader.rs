use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub mod racelist;

// pub trait Reader<T> where T: super::WebPage,

pub trait Reader {
    //fn open(&self) -> T;
    fn get_file_path(&self) -> std::path::PathBuf;
    fn get_file_dir_path(&self) -> std::path::PathBuf;
    fn get_url(&self) -> String;
    fn fetch_string(&self) -> String {
        let url = self.get_url();
        log::info!("fetching {}", url);
        let res = reqwest::blocking::get(url).unwrap();
        log::info!("Response: {:?} {}", &res.version(), &res.status());
        res.text().unwrap()
    }
    fn open_string(&self) -> String {
        todo!()
    }
    fn save_string(&self, text: &str) {
        fs::create_dir_all(self.get_file_dir_path()).unwrap();

        let text_bin = text.as_bytes();

        let mut e = GzEncoder::new(Vec::new(), Compression::default());
        e.write_all(&text_bin).unwrap();
        let foo = e.finish().unwrap();
        
        let mut file = File::create(self.get_file_path()).unwrap();
        file.write_all( &foo).unwrap();
    }
}
