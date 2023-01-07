pub mod racelist;
pub mod race;

pub trait WebPage{
    fn save_to_file(&self) -> ();
    fn save_to_db(&self) -> ();
}