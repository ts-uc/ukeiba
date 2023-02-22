use super::WebPageTrait;
use crate::common::horse_birthdate_parents::HorseBirthdateParents;
use crate::DbType;
use anyhow::Result;
use std::path::PathBuf;

pub struct BajikyoSearchPage(pub HorseBirthdateParents);

impl WebPageTrait for BajikyoSearchPage {
    fn get_path(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap()
            .join("ukeiba")
            .join("bajikyo_search")
            .join(self.0.horse.get_upper_id().to_string())
            .join(format!(
                "bajikyo_search_{}.json.gz",
                self.0.horse.get_horse_id()
            ))
    }
    fn fetch(&self) -> Result<String> {
        todo!()
    }
    fn scrap(&self, body: &str) -> Vec<DbType> {
        todo!()
    }
}
