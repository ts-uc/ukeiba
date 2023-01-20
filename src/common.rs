use std::path::PathBuf;

pub mod date_racecourse;
pub mod race;
pub mod race_horse;
pub mod horse;
pub mod racecourse;

pub trait GetPath {
    fn get_dir_path(&self) -> PathBuf;
    fn get_data_id(&self) -> String;
}