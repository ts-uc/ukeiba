use crate::common::*;
use crate::db::{
    writer::{write_to_db, DbWriter},
    Horses,
};
use crate::get::get_horse_bajikyo_id;
use chrono::Datelike;
use ukeiba_common::scraper::bajikyo_profile;

pub fn scrap() {
    let horse_bajikyo_ids = get_horse_bajikyo_id::get_from_db();

    let bajikyo_profile_pages = horse_bajikyo_ids
        .iter()
        .map(|x| bajikyo_profile::Page {
            horse_bajikyo_id: x.0.clone(),
        })
        .collect::<Vec<_>>();

    let bajikyo_profile_data = fetch_and_scrap_all(bajikyo_profile_pages);

    let horse_data = bajikyo_profile_data
        .into_iter()
        .map(|data| Horses {
            horse_bajikyo_id: Some(data.horse_bajikyo_id),
            horse_birthdate: data.horse_birthdate,
            horse_birth_year: data.horse_birthdate.map(|x| x.year()),
            horse_coat_color: data.horse_coat_color,
            horse_breed: data.horse_breed,
            breeder: data.horse_breeder,
            breeder_location: data.horse_breeder_address,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    let db_writer = horse_data
        .into_iter()
        .map(|x| DbWriter::BajikyoProfileToHorses(x))
        .collect::<Vec<_>>();

    write_to_db(&db_writer);
}
