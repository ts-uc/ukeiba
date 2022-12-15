pub mod racelist;

pub trait WebPage{
    fn save_to_file(&self) -> ();
    fn save_to_db(&self) -> ();
}