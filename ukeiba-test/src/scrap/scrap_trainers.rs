use crate::common::*;
use crate::db::{
    writer::{write_to_db, DbWriter},
    Trainers,
};
use crate::get::get_trainer_nar_id;
use ukeiba_common::scraper::trainer_profile;

pub fn scrap() {
    let trainer_all_ids = get_trainer_nar_id::get_all_from_nar();

    let trainer_profile_pages = trainer_all_ids
        .into_iter()
        .map(|trainer_nar_id| trainer_profile::Page {
            trainer_nar_id: trainer_nar_id.0,
        })
        .collect::<Vec<_>>();

    let trainer_profile_pages = fetch_and_scrap_all(trainer_profile_pages);

    //DBへ書き込むデータを作成

    let trainers = trainer_profile_pages
        .into_iter()
        .map(|data| Trainers {
            trainer_nar_id: data.trainer_nar_id,
            trainer_name: data.name,
            trainer_kana: data.kana,
            trainer_sex: data.sex,
            trainer_status: data.status,
            trainer_birthdate: data.birthdate,
            trainer_first_run: data.first_run,
            trainer_first_win: data.first_win,
        })
        .collect::<Vec<_>>();

    let db_writer = trainers
        .into_iter()
        .map(|x| DbWriter::TrainersToTrainers(x))
        .collect::<Vec<_>>();

    write_to_db(&db_writer);
}
