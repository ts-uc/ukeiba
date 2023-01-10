pub mod racelist;
pub mod race;
pub mod horse_history;
pub mod horse_profile;

pub trait WebPage{
    fn save_to_file(&self) -> ();
    fn save_to_db(&self) -> ();
}