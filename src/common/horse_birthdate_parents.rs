use crate::Horse;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct HorseBirthdateParents {
    pub horse: Horse,
    pub horse_name: String,
    pub birthdate: NaiveDate,
    pub sire_name: String,
    pub dam_name: String,
}
