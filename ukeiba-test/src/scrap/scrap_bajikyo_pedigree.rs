use crate::common::*;
use crate::db::{
    writer::{write_to_db, DbWriter},
    Horses,
};
use crate::get::get_horse_bajikyo_id;
use ukeiba_common::scraper::bajikyo_pedigree;

pub fn scrap() {
    let horse_bajikyo_ids = get_horse_bajikyo_id::get_from_db();

    let bajikyo_pedigree_pages = horse_bajikyo_ids
        .iter()
        .map(|x| bajikyo_pedigree::Page {
            horse_bajikyo_id: x.clone(),
        })
        .collect::<Vec<_>>();

    let bajikyo_pedigree_data = fetch_and_scrap_all(bajikyo_pedigree_pages);
    let horse_data = bajikyo_pedigree_data
        .into_iter()
        .map(|data| Horses {
            horse_bajikyo_id: Some(data.horse_bajikyo_id),
            sire_bajikyo_id: data.sire_bajikyo_id,
            dam_bajikyo_id: data.dam_bajikyo_id,
            bms_bajikyo_id: data.bms_bajikyo_id,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    let db_writer = horse_data
        .into_iter()
        .map(|x| DbWriter::BajikyoPedigreeToHorses(x))
        .collect::<Vec<_>>();

    write_to_db(&db_writer);
}
