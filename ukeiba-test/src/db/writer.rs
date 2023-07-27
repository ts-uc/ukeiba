use super::*;
use rusqlite::Transaction;
use serde_rusqlite::to_params_named;

pub fn write_to_db(db_writers: &[DbWriter]) {
    let mut conn = make_conn().unwrap();
    let tx = conn.transaction().unwrap();
    for datum in db_writers {
        datum.execute(&tx)
    }
    tx.commit().unwrap();
}

pub enum DbWriter {
    UpsertDates(Dates),
    RaceListToRaces(Races),
    HorseHistoryToRaces(Races),
    HorseTableToRaces(Races),
    HorseHistoryToRaceHorses(RaceHorses),
    HorseTableToRaceHorses(RaceHorses),
    BajikyoPedigreeToHorses(Horses),
    BajikyoProfileToHorses(Horses),
    HorseHistoryToHorses(Horses),
    HorseProfileToHorses(Horses),
    JockeysToJockeys(Jockeys),
    TrainersToTrainers(Trainers),
}

impl DbWriter {
    fn execute(&self, tx: &Transaction) {
        match self {
            Self::UpsertDates(datum) => upsert_dates(tx, &datum),
            Self::RaceListToRaces(datum) => race_list_to_races(tx, &datum),
            Self::HorseHistoryToRaces(datum) => horse_history_to_races(tx, &datum),
            Self::HorseTableToRaces(datum) => horse_table_to_races(tx, &datum),
            Self::HorseHistoryToRaceHorses(datum) => horse_history_to_race_horses(tx, &datum),
            Self::HorseTableToRaceHorses(datum) => horse_table_to_race_horses(tx, &datum),
            Self::BajikyoPedigreeToHorses(datum) => bajikyo_pedigree_to_horses(tx, &datum),
            Self::BajikyoProfileToHorses(datum) => bajikyo_profile_to_horses(tx, &datum),
            Self::HorseHistoryToHorses(datum) => horse_history_to_horses(tx, &datum),
            Self::HorseProfileToHorses(datum) => horse_profile_to_horses(tx, &datum),
            Self::JockeysToJockeys(datum) => jockeys_to_jockeys(tx, &datum),
            Self::TrainersToTrainers(datum) => trainers_to_trainers(tx, &datum),
        }
    }
}

// テンプレート

// fn upsert_races(tx: &Transaction, datum: &Races) {
//     tx.execute(
//         "
//         INSERT INTO races (
//             race_date, race_num, post_time, post_time_change, race_sub_name, race_name,
//             weather, going, race_class, race_kumi, race_mixed, race_age, race_sex,
//             race_horse_select_type, race_weight_type, race_type, horse_count_run,
//             horse_count_entered, race_align
//         )
//         VALUES (
//             :race_date, :race_num, :post_time, :post_time_change, :race_sub_name, :race_name,
//             :weather, :going, :race_class, :race_kumi, :race_mixed, :race_age, :race_sex,
//             :race_horse_select_type, :race_weight_type, :race_type, :horse_count_run,
//             :horse_count_entered, :race_align
//         )
//         ON CONFLICT(race_date, race_num) DO UPDATE SET
//             post_time = COALESCE(:post_time, races.post_time),
//             post_time_change = COALESCE(:post_time_change, races.post_time_change),
//             race_sub_name = COALESCE(:race_sub_name, races.race_sub_name),
//             race_name = COALESCE(:race_name, races.race_name),
//             weather = COALESCE(:weather, races.weather),
//             going = COALESCE(:going, races.going),
//             race_class = COALESCE(:race_class, races.race_class),
//             race_kumi = COALESCE(:race_kumi, races.race_kumi),
//             race_mixed = COALESCE(:race_mixed, races.race_mixed),
//             race_age = COALESCE(:race_age, races.race_age),
//             race_sex = COALESCE(:race_sex, races.race_sex),
//             race_horse_select_type = COALESCE(:race_horse_select_type, races.race_horse_select_type),
//             race_weight_type = COALESCE(:race_weight_type, races.race_weight_type),
//             race_type = COALESCE(:race_type, races.race_type),
//             horse_count_run = COALESCE(:horse_count_run, races.horse_count_run),
//             horse_count_entered = COALESCE(:horse_count_entered, races.horse_count_entered),
//             race_align = COALESCE(:race_align, races.race_align)
//     ",
//         to_params_named(&datum).unwrap().to_slice().as_slice(),
//     )
//     .unwrap();
// }

// fn upsert_race_horses(tx: &Transaction, datum: &RaceHorses) {
//     tx.execute(
//         "
//         INSERT INTO race_horses (
//             race_date, race_num, horse_num, horse_nar_id, bracket_num, gate_num,
//             horse_sex, jockey_nar_id, weight_mark, weight_to_carry, trainer_nar_id,
//             owner_name, horse_weight, change, win_fav, arrival, arrival_info,
//             finish_time, prize, win_odds, place_odds_min, place_odds_max
//         )
//         VALUES (
//             :race_date, :race_num, :horse_num, :horse_nar_id, :bracket_num, :gate_num,
//             :horse_sex, :jockey_nar_id, :weight_mark, :weight_to_carry, :trainer_nar_id,
//             :owner_name, :horse_weight, :change, :win_fav, :arrival, :arrival_info,
//             :finish_time, :prize, :win_odds, :place_odds_min, :place_odds_max
//         )
//         ON CONFLICT(race_date, race_num, horse_num) DO UPDATE SET
//             horse_nar_id = COALESCE(:horse_nar_id, race_horses.horse_nar_id),
//             bracket_num = COALESCE(:bracket_num, race_horses.bracket_num),
//             gate_num = COALESCE(:gate_num, race_horses.gate_num),
//             horse_sex = COALESCE(:horse_sex, race_horses.horse_sex),
//             jockey_nar_id = COALESCE(:jockey_nar_id, race_horses.jockey_nar_id),
//             weight_mark = COALESCE(:weight_mark, race_horses.weight_mark),
//             weight_to_carry = COALESCE(:weight_to_carry, race_horses.weight_to_carry),
//             trainer_nar_id = COALESCE(:trainer_nar_id, race_horses.trainer_nar_id),
//             owner_name = COALESCE(:owner_name, race_horses.owner_name),
//             horse_weight = COALESCE(:horse_weight, race_horses.horse_weight),
//             change = COALESCE(:change, race_horses.change),
//             win_fav = COALESCE(:win_fav, race_horses.win_fav),
//             arrival = COALESCE(:arrival, race_horses.arrival),
//             arrival_info = COALESCE(:arrival_info, race_horses.arrival_info),
//             finish_time = COALESCE(:finish_time, race_horses.finish_time),
//             prize = COALESCE(:prize, race_horses.prize),
//             win_odds = COALESCE(:win_odds, race_horses.win_odds),
//             place_odds_min = COALESCE(:place_odds_min, race_horses.place_odds_min),
//             place_odds_max = COALESCE(:place_odds_max, race_horses.place_odds_max)
//     ",
//         to_params_named(&datum).unwrap().to_slice().as_slice(),
//     )
//     .unwrap();
// }

// fn upsert_horses(tx: &Transaction, datum: &Horses) {
//     tx.execute(
//         "
//         INSERT INTO horses (
//             horse_bajikyo_id, horse_nar_id, horse_name, horse_status,
//             deregistration_date, horse_birthdate, horse_coat_color, horse_breed,
//             breeder, breeder_location, sire_bajikyo_id, dam_bajikyo_id, bms_bajikyo_id
//         )
//         VALUES (
//             :horse_bajikyo_id, :horse_nar_id, :horse_name, :horse_status,
//             :deregistration_date, :horse_birthdate, :horse_coat_color, :horse_breed,
//             :breeder, :breeder_location, :sire_bajikyo_id, :dam_bajikyo_id, :bms_bajikyo_id
//         )
//         ON CONFLICT(horse_bajikyo_id) DO UPDATE SET
//             horse_nar_id = COALESCE(:horse_nar_id, horses.horse_nar_id),
//             horse_name = COALESCE(:horse_name, horses.horse_name),
//             horse_status = COALESCE(:horse_status, horses.horse_status),
//             deregistration_date = COALESCE(:deregistration_date, horses.deregistration_date),
//             horse_birthdate = COALESCE(:horse_birthdate, horses.horse_birthdate),
//             horse_coat_color = COALESCE(:horse_coat_color, horses.horse_coat_color),
//             horse_breed = COALESCE(:horse_breed, horses.horse_breed),
//             breeder = COALESCE(:breeder, horses.breeder),
//             breeder_location = COALESCE(:breeder_location, horses.breeder_location),
//             sire_bajikyo_id = COALESCE(:sire_bajikyo_id, horses.sire_bajikyo_id),
//             dam_bajikyo_id = COALESCE(:dam_bajikyo_id, horses.dam_bajikyo_id),
//             bms_bajikyo_id = COALESCE(:bms_bajikyo_id, horses.bms_bajikyo_id)
//     ",
//         to_params_named(&datum).unwrap().to_slice().as_slice(),
//     )
//     .unwrap();
// }

// fn upsert_jockeys(tx: &Transaction, datum: &Jockeys) {
//     tx.execute(
//         "
//         INSERT INTO jockeys (
//             jockey_nar_id, jockey_name, jockey_kana, jockey_sex,
//             jockey_status, jockey_birthdate, jockey_first_run, jockey_first_win
//         )
//         VALUES (
//             :jockey_nar_id, :jockey_name, :jockey_kana, :jockey_sex,
//             :jockey_status, :jockey_birthdate, :jockey_first_run, :jockey_first_win
//         )
//         ON CONFLICT(jockey_nar_id) DO UPDATE SET
//             jockey_name = COALESCE(:jockey_name, jockeys.jockey_name),
//             jockey_kana = COALESCE(:jockey_kana, jockeys.jockey_kana),
//             jockey_sex = COALESCE(:jockey_sex, jockeys.jockey_sex),
//             jockey_status = COALESCE(:jockey_status, jockeys.jockey_status),
//             jockey_birthdate = COALESCE(:jockey_birthdate, jockeys.jockey_birthdate),
//             jockey_first_run = COALESCE(:jockey_first_run, jockeys.jockey_first_run),
//             jockey_first_win = COALESCE(:jockey_first_win, jockeys.jockey_first_win)
//     ",
//         to_params_named(&datum).unwrap().to_slice().as_slice(),
//     )
//     .unwrap();
// }

// fn upsert_trainers(tx: &Transaction, datum: &Trainers) {
//     tx.execute(
//         "
//         INSERT INTO trainers (
//             trainer_nar_id, trainer_name, trainer_kana, trainer_sex,
//             trainer_status, trainer_birthdate, trainer_first_run, trainer_first_win
//         )
//         VALUES (
//             :trainer_nar_id, :trainer_name, :trainer_kana, :trainer_sex,
//             :trainer_status, :trainer_birthdate, :trainer_first_run, :trainer_first_win
//         )
//         ON CONFLICT(trainer_nar_id) DO UPDATE SET
//             trainer_name = COALESCE(:trainer_name, trainers.trainer_name),
//             trainer_kana = COALESCE(:trainer_kana, trainers.trainer_kana),
//             trainer_sex = COALESCE(:trainer_sex, trainers.trainer_sex),
//             trainer_status = COALESCE(:trainer_status, trainers.trainer_status),
//             trainer_birthdate = COALESCE(:trainer_birthdate, trainers.trainer_birthdate),
//             trainer_first_run = COALESCE(:trainer_first_run, trainers.trainer_first_run),
//             trainer_first_win = COALESCE(:trainer_first_win, trainers.trainer_first_win)
//     ",
//         to_params_named(&datum).unwrap().to_slice().as_slice(),
//     )
//     .unwrap();
// }

// fn upsert_races(tx: &Transaction, datum: &Races) {
//     tx.execute(
//         "
//         INSERT INTO races (race_date, race_num, post_time, post_time_change, race_sub_name, race_name, weather, going, race_class, race_kumi, race_mixed, race_age, race_sex, race_horse_select_type, race_weight_type, race_type, horse_count_run, horse_count_entered, race_align)
//         VALUES (:race_date, :race_num, :post_time, :post_time_change, :race_sub_name, :race_name, :weather, :going, :race_class, :race_kumi, :race_mixed, :race_age, :race_sex, :race_horse_select_type, :race_weight_type, :race_type, :horse_count_run, :horse_count_entered, :race_align)
//         ON CONFLICT(race_date, race_num) DO UPDATE SET
//             post_time = COALESCE(races.post_time, :post_time),
//             post_time_change = COALESCE(races.post_time_change, :post_time_change),
//             race_sub_name = COALESCE(races.race_sub_name, :race_sub_name),
//             race_name = COALESCE(races.race_name, :race_name),
//             weather = COALESCE(races.weather, :weather),
//             going = COALESCE(races.going, :going),
//             race_class = COALESCE(races.race_class, :race_class),
//             race_kumi = COALESCE(races.race_kumi, :race_kumi),
//             race_mixed = COALESCE(races.race_mixed, :race_mixed),
//             race_age = COALESCE(races.race_age, :race_age),
//             race_sex = COALESCE(races.race_sex, :race_sex),
//             race_horse_select_type = COALESCE(races.race_horse_select_type, :race_horse_select_type),
//             race_weight_type = COALESCE(races.race_weight_type, :race_weight_type),
//             race_type = COALESCE(races.race_type, :race_type),
//             horse_count_run = COALESCE(races.horse_count_run, :horse_count_run),
//             horse_count_entered = COALESCE(races.horse_count_entered, :horse_count_entered),
//             race_align = COALESCE(races.race_align, :race_align)
//     ",
//         to_params_named(&datum).unwrap().to_slice().as_slice(),
//     )
//     .unwrap();
// }

// fn upsert_race_horses(tx: &Transaction, datum: &RaceHorses) {
//     tx.execute(
//         "
//         INSERT INTO race_horses (race_date, race_num, horse_num, horse_nar_id, bracket_num, gate_num, horse_sex, jockey_nar_id, weight_mark, weight_to_carry, trainer_nar_id, owner_name, horse_weight, change, win_fav, arrival, arrival_info, finish_time, prize, win_odds, place_odds_min, place_odds_max)
//         VALUES (:race_date, :race_num, :horse_num, :horse_nar_id, :bracket_num, :gate_num, :horse_sex, :jockey_nar_id, :weight_mark, :weight_to_carry, :trainer_nar_id, :owner_name, :horse_weight, :change, :win_fav, :arrival, :arrival_info, :finish_time, :prize, :win_odds, :place_odds_min, :place_odds_max)
//         ON CONFLICT(race_date, race_num, horse_num) DO UPDATE SET
//             horse_nar_id = COALESCE(race_horses.horse_nar_id, :horse_nar_id),
//             bracket_num = COALESCE(race_horses.bracket_num, :bracket_num),
//             gate_num = COALESCE(race_horses.gate_num, :gate_num),
//             horse_sex = COALESCE(race_horses.horse_sex, :horse_sex),
//             jockey_nar_id = COALESCE(race_horses.jockey_nar_id, :jockey_nar_id),
//             weight_mark = COALESCE(race_horses.weight_mark, :weight_mark),
//             weight_to_carry = COALESCE(race_horses.weight_to_carry, :weight_to_carry),
//             trainer_nar_id = COALESCE(race_horses.trainer_nar_id, :trainer_nar_id),
//             owner_name = COALESCE(race_horses.owner_name, :owner_name),
//             horse_weight = COALESCE(race_horses.horse_weight, :horse_weight),
//             change = COALESCE(race_horses.change, :change),
//             win_fav = COALESCE(race_horses.win_fav, :win_fav),
//             arrival = COALESCE(race_horses.arrival, :arrival),
//             arrival_info = COALESCE(race_horses.arrival_info, :arrival_info),
//             finish_time = COALESCE(race_horses.finish_time, :finish_time),
//             prize = COALESCE(race_horses.prize, :prize),
//             win_odds = COALESCE(race_horses.win_odds, :win_odds),
//             place_odds_min = COALESCE(race_horses.place_odds_min, :place_odds_min),
//             place_odds_max = COALESCE(race_horses.place_odds_max, :place_odds_max)
//     ",
//         to_params_named(&datum).unwrap().to_slice().as_slice(),
//     )
//     .unwrap();
// }

// fn upsert_horses(tx: &Transaction, datum: &Horses) {
//     tx.execute(
//         "
//         INSERT INTO horses (horse_bajikyo_id, horse_nar_id, horse_name, horse_status, deregistration_date, horse_birthdate, horse_coat_color, horse_breed, breeder, breeder_location, sire_bajikyo_id, dam_bajikyo_id, bms_bajikyo_id)
//         VALUES (:horse_bajikyo_id, :horse_nar_id, :horse_name, :horse_status, :deregistration_date, :horse_birthdate, :horse_coat_color, :horse_breed, :breeder, :breeder_location, :sire_bajikyo_id, :dam_bajikyo_id, :bms_bajikyo_id)
//         ON CONFLICT(horse_bajikyo_id) DO UPDATE SET
//             horse_nar_id = COALESCE(horses.horse_nar_id, :horse_nar_id),
//             horse_name = COALESCE(horses.horse_name, :horse_name),
//             horse_status = COALESCE(horses.horse_status, :horse_status),
//             deregistration_date = COALESCE(horses.deregistration_date, :deregistration_date),
//             horse_birthdate = COALESCE(horses.horse_birthdate, :horse_birthdate),
//             horse_coat_color = COALESCE(horses.horse_coat_color, :horse_coat_color),
//             horse_breed = COALESCE(horses.horse_breed, :horse_breed),
//             breeder = COALESCE(horses.breeder, :breeder),
//             breeder_location = COALESCE(horses.breeder_location, :breeder_location),
//             sire_bajikyo_id = COALESCE(horses.sire_bajikyo_id, :sire_bajikyo_id),
//             dam_bajikyo_id = COALESCE(horses.dam_bajikyo_id, :dam_bajikyo_id),
//             bms_bajikyo_id = COALESCE(horses.bms_bajikyo_id, :bms_bajikyo_id)
//     ",
//         to_params_named(&datum).unwrap().to_slice().as_slice(),
//     )
//     .unwrap();
// }

// fn upsert_jockeys(tx: &Transaction, datum: &Jockeys) {
//     tx.execute(
//         "
//         INSERT INTO jockeys (jockey_nar_id, jockey_name, jockey_kana, jockey_sex, jockey_status, jockey_birthdate, jockey_first_run, jockey_first_win)
//         VALUES (:jockey_nar_id, :jockey_name, :jockey_kana, :jockey_sex, :jockey_status, :jockey_birthdate, :jockey_first_run, :jockey_first_win)
//         ON CONFLICT(jockey_nar_id) DO UPDATE SET
//             jockey_name = COALESCE(jockeys.jockey_name, :jockey_name),
//             jockey_kana = COALESCE(jockeys.jockey_kana, :jockey_kana),
//             jockey_sex = COALESCE(jockeys.jockey_sex, :jockey_sex),
//             jockey_status = COALESCE(jockeys.jockey_status, :jockey_status),
//             jockey_birthdate = COALESCE(jockeys.jockey_birthdate, :jockey_birthdate),
//             jockey_first_run = COALESCE(jockeys.jockey_first_run, :jockey_first_run),
//             jockey_first_win = COALESCE(jockeys.jockey_first_win, :jockey_first_win)
//     ",
//         to_params_named(&datum).unwrap().to_slice().as_slice(),
//     )
//     .unwrap();
// }

// fn upsert_trainers(tx: &Transaction, datum: &Trainers) {
//     tx.execute(
//         "
//         INSERT INTO trainers (trainer_nar_id, trainer_name, trainer_kana, trainer_sex, trainer_status, trainer_birthdate, trainer_first_run, trainer_first_win)
//         VALUES (:trainer_nar_id, :trainer_name, :trainer_kana, :trainer_sex, :trainer_status, :trainer_birthdate, :trainer_first_run, :trainer_first_win)
//         ON CONFLICT(trainer_nar_id) DO UPDATE SET
//             trainer_name = COALESCE(trainers.trainer_name, :trainer_name),
//             trainer_kana = COALESCE(trainers.trainer_kana, :trainer_kana),
//             trainer_sex = COALESCE(trainers.trainer_sex, :trainer_sex),
//             trainer_status = COALESCE(trainers.trainer_status, :trainer_status),
//             trainer_birthdate = COALESCE(trainers.trainer_birthdate, :trainer_birthdate),
//             trainer_first_run = COALESCE(trainers.trainer_first_run, :trainer_first_run),
//             trainer_first_win = COALESCE(trainers.trainer_first_win, :trainer_first_win)
//     ",
//         to_params_named(&datum).unwrap().to_slice().as_slice(),
//     )
//     .unwrap();
// }

fn upsert_dates(tx: &Transaction, datum: &Dates) {
    tx.execute(
        "
        INSERT INTO dates (
            race_date, racecourse, fiscal_year, kai, nichi, capability_test, heating, sand_obstacle
        )
        VALUES (
            :race_date, :racecourse, :fiscal_year, :kai, :nichi, :capability_test, :heating, :sand_obstacle
        )
        ON CONFLICT(race_date) DO UPDATE SET
            racecourse = COALESCE(:racecourse, dates.racecourse),
            fiscal_year = COALESCE(:fiscal_year, dates.fiscal_year),
            kai = COALESCE(:kai, dates.kai),
            nichi = COALESCE(:nichi, dates.nichi),
            capability_test = COALESCE(:capability_test, dates.capability_test),
            heating = COALESCE(:heating, dates.heating),
            sand_obstacle = COALESCE(:sand_obstacle, dates.sand_obstacle)
        ",
        to_params_named(&datum).unwrap().to_slice().as_slice(),
    )
    .unwrap();
}

fn race_list_to_races(tx: &Transaction, datum: &Races) {
    tx.execute(
        "
        INSERT INTO races (race_date, race_num, post_time, post_time_change, race_sub_name, race_name, race_detail, weather, going, race_class, race_kumi, race_class_mixed, race_kumi_mixed, race_final, race_age, race_sex, race_horse_select_type, race_weight_type, race_type, horse_count_run, horse_count_entered, race_align)
        VALUES (:race_date, :race_num, :post_time, :post_time_change, :race_sub_name, :race_name, :race_detail, :weather, :going, :race_class, :race_kumi, :race_class_mixed, :race_kumi_mixed, :race_final, :race_age, :race_sex, :race_horse_select_type, :race_weight_type, :race_type, :horse_count_run, :horse_count_entered, :race_align)
        ON CONFLICT(race_date, race_num) DO UPDATE SET
            post_time = COALESCE(races.post_time, :post_time),
            post_time_change = COALESCE(races.post_time_change, :post_time_change),
            race_sub_name = COALESCE(races.race_sub_name, :race_sub_name),
            race_name = COALESCE(races.race_name, :race_name),
            race_detail = COALESCE(races.race_detail, :race_detail),
            weather = COALESCE(races.weather, :weather),
            going = COALESCE(races.going, :going),
            race_class = COALESCE(races.race_class, :race_class),
            race_kumi = COALESCE(races.race_kumi, :race_kumi),
            race_class_mixed = COALESCE(races.race_class_mixed, :race_class_mixed),
            race_kumi_mixed = COALESCE(races.race_kumi_mixed, :race_kumi_mixed),
            race_final = COALESCE(races.race_final, :race_final),
            race_age = COALESCE(races.race_age, :race_age),
            race_sex = COALESCE(races.race_sex, :race_sex),
            race_horse_select_type = COALESCE(races.race_horse_select_type, :race_horse_select_type),
            race_weight_type = COALESCE(races.race_weight_type, :race_weight_type),
            race_type = COALESCE(races.race_type, :race_type),
            horse_count_run = COALESCE(races.horse_count_run, :horse_count_run),
            horse_count_entered = COALESCE(races.horse_count_entered, :horse_count_entered),
            race_align = COALESCE(races.race_align, :race_align)
    ",
to_params_named(&datum).unwrap().to_slice().as_slice(),
    )
    .unwrap();
}

fn horse_history_to_races(tx: &Transaction, datum: &Races) {
    tx.execute(
        "
        INSERT INTO races (race_date, race_num, post_time, post_time_change, race_sub_name, race_name, race_detail, weather, going, race_class, race_kumi, race_class_mixed, race_kumi_mixed, race_final, race_age, race_sex, race_horse_select_type, race_weight_type, race_type, horse_count_run, horse_count_entered, race_align)
        VALUES (:race_date, :race_num, :post_time, :post_time_change, :race_sub_name, :race_name, :race_detail, :weather, :going, :race_class, :race_kumi, :race_class_mixed, :race_kumi_mixed, :race_final, :race_age, :race_sex, :race_horse_select_type, :race_weight_type, :race_type, :horse_count_run, :horse_count_entered, :race_align)
        ON CONFLICT(race_date, race_num) DO UPDATE SET
            post_time = COALESCE(races.post_time, :post_time),
            post_time_change = COALESCE(races.post_time_change, :post_time_change),
            race_sub_name = COALESCE(races.race_sub_name, :race_sub_name),
            race_name = COALESCE(races.race_name, :race_name),
            race_detail = COALESCE(races.race_detail, :race_detail),
            weather = COALESCE(races.weather, :weather),
            going = COALESCE(races.going, :going),
            race_class = COALESCE(races.race_class, :race_class),
            race_kumi = COALESCE(races.race_kumi, :race_kumi),
            race_class_mixed = COALESCE(races.race_class_mixed, :race_class_mixed),
            race_kumi_mixed = COALESCE(races.race_kumi_mixed, :race_kumi_mixed),
            race_final = COALESCE(races.race_final, :race_final),
            race_age = COALESCE(races.race_age, :race_age),
            race_sex = COALESCE(races.race_sex, :race_sex),
            race_horse_select_type = COALESCE(races.race_horse_select_type, :race_horse_select_type),
            race_weight_type = COALESCE(races.race_weight_type, :race_weight_type),
            race_type = COALESCE(:race_type, races.race_type),
            horse_count_run = COALESCE(:horse_count_run, races.horse_count_run),
            horse_count_entered = COALESCE(races.horse_count_entered, :horse_count_entered),
            race_align = COALESCE(races.race_align, :race_align)
    ",
to_params_named(&datum).unwrap().to_slice().as_slice(),
    )
    .unwrap();
}

fn horse_table_to_races(tx: &Transaction, datum: &Races) {
    tx.execute(
        "
        INSERT INTO races (race_date, race_num, post_time, post_time_change, race_sub_name, race_name, race_detail, weather, going, race_class, race_kumi, race_class_mixed, race_kumi_mixed, race_final, race_age, race_sex, race_horse_select_type, race_weight_type, race_type, horse_count_run, horse_count_entered, race_align)
        VALUES (:race_date, :race_num, :post_time, :post_time_change, :race_sub_name, :race_name, :race_detail, :weather, :going, :race_class, :race_kumi, :race_class_mixed, :race_kumi_mixed, :race_final, :race_age, :race_sex, :race_horse_select_type, :race_weight_type, :race_type, :horse_count_run, :horse_count_entered, :race_align)
        ON CONFLICT(race_date, race_num) DO UPDATE SET
            post_time = COALESCE(:post_time, races.post_time),
            post_time_change = COALESCE(:post_time_change, races.post_time_change),
            race_sub_name = COALESCE(:race_sub_name, races.race_sub_name),
            race_name = :race_name,
            race_detail = COALESCE(:race_detail, races.race_detail),
            weather = COALESCE(:weather, races.weather),
            going = COALESCE(:going, races.going),
            race_class = COALESCE(:race_class, races.race_class),
            race_kumi = COALESCE(:race_kumi, races.race_kumi),
            race_class_mixed = COALESCE(:race_class_mixed, races.race_class_mixed),
            race_kumi_mixed = COALESCE(:race_kumi_mixed, races.race_kumi_mixed),
            race_final = COALESCE(:race_final, races.race_final),
            race_age = COALESCE(:race_age, races.race_age),
            race_sex = COALESCE(:race_sex, races.race_sex),
            race_horse_select_type = COALESCE(:race_horse_select_type, races.race_horse_select_type),
            race_weight_type = COALESCE(:race_weight_type, races.race_weight_type),
            race_type = COALESCE(races.race_type, :race_type),
            horse_count_run = COALESCE(races.horse_count_run, :horse_count_run),
            horse_count_entered = COALESCE(:horse_count_entered, races.horse_count_entered),
            race_align = COALESCE(races.race_align, :race_align)
                ",
to_params_named(&datum).unwrap().to_slice().as_slice(),
    )
    .unwrap();
}

fn horse_history_to_race_horses(tx: &Transaction, datum: &RaceHorses) {
    tx.execute(
        "
        INSERT INTO race_horses (race_date, race_num, horse_num, horse_nar_id, bracket_num, gate_num, horse_sex, jockey_nar_id, weight_mark, weight_to_carry, trainer_nar_id, owner_name, horse_weight, change, win_fav, arrival, arrival_info, finish_time, prize, win_odds, place_odds_min, place_odds_max)
        VALUES (:race_date, :race_num, :horse_num, :horse_nar_id, :bracket_num, :gate_num, :horse_sex, :jockey_nar_id, :weight_mark, :weight_to_carry, :trainer_nar_id, :owner_name, :horse_weight, :change, :win_fav, :arrival, :arrival_info, :finish_time, :prize, :win_odds, :place_odds_min, :place_odds_max)
        ON CONFLICT(race_date, race_num, horse_num) DO UPDATE SET
            horse_nar_id = COALESCE(race_horses.horse_nar_id, :horse_nar_id),
            bracket_num = COALESCE(race_horses.bracket_num, :bracket_num),
            gate_num = COALESCE(race_horses.gate_num, :gate_num),
            horse_sex = COALESCE(race_horses.horse_sex, :horse_sex),
            jockey_nar_id = COALESCE(race_horses.jockey_nar_id, :jockey_nar_id),
            weight_mark = COALESCE(race_horses.weight_mark, :weight_mark),
            weight_to_carry = COALESCE(race_horses.weight_to_carry, :weight_to_carry),
            trainer_nar_id = COALESCE(race_horses.trainer_nar_id, :trainer_nar_id),
            owner_name = COALESCE(race_horses.owner_name, :owner_name),
            horse_weight = COALESCE(race_horses.horse_weight, :horse_weight),
            change = COALESCE(race_horses.change, :change),
            win_fav = COALESCE(:win_fav, race_horses.win_fav),
            arrival = COALESCE(:arrival, race_horses.arrival),
            arrival_info = COALESCE(:arrival_info, race_horses.arrival_info),
            finish_time = COALESCE(:finish_time, race_horses.finish_time),
            prize = COALESCE(:prize, race_horses.prize),
            win_odds = COALESCE(race_horses.win_odds, :win_odds),
            place_odds_min = COALESCE(race_horses.place_odds_min, :place_odds_min),
            place_odds_max = COALESCE(race_horses.place_odds_max, :place_odds_max)
    ",
to_params_named(&datum).unwrap().to_slice().as_slice(),
    )
    .unwrap();
}

fn horse_table_to_race_horses(tx: &Transaction, datum: &RaceHorses) {
    tx.execute(
        "
        INSERT INTO race_horses (race_date, race_num, horse_num, horse_nar_id, bracket_num, gate_num, horse_sex, jockey_nar_id, weight_mark, weight_to_carry, trainer_nar_id, owner_name, horse_weight, change, win_fav, arrival, arrival_info, finish_time, prize, win_odds, place_odds_min, place_odds_max)
        VALUES (:race_date, :race_num, :horse_num, :horse_nar_id, :bracket_num, :gate_num, :horse_sex, :jockey_nar_id, :weight_mark, :weight_to_carry, :trainer_nar_id, :owner_name, :horse_weight, :change, :win_fav, :arrival, :arrival_info, :finish_time, :prize, :win_odds, :place_odds_min, :place_odds_max)
        ON CONFLICT(race_date, race_num, horse_num) DO UPDATE SET
            horse_nar_id = COALESCE(:horse_nar_id, race_horses.horse_nar_id),
            bracket_num = COALESCE(:bracket_num, race_horses.bracket_num),
            gate_num = COALESCE(:gate_num, race_horses.gate_num),
            horse_sex = COALESCE(:horse_sex, race_horses.horse_sex),
            jockey_nar_id = COALESCE(:jockey_nar_id, race_horses.jockey_nar_id),
            weight_mark = COALESCE(:weight_mark, race_horses.weight_mark),
            weight_to_carry = COALESCE(:weight_to_carry, race_horses.weight_to_carry),
            trainer_nar_id = COALESCE(:trainer_nar_id, race_horses.trainer_nar_id),
            owner_name = COALESCE(:owner_name, race_horses.owner_name),
            horse_weight = COALESCE(:horse_weight, race_horses.horse_weight),
            change = COALESCE(:change, race_horses.change),
            win_fav = COALESCE(race_horses.win_fav, :win_fav),
            arrival = COALESCE(race_horses.arrival, :arrival),
            arrival_info = COALESCE(race_horses.arrival_info, :arrival_info),
            finish_time = COALESCE(race_horses.finish_time, :finish_time),
            prize = COALESCE(race_horses.prize, :prize),
            win_odds = COALESCE(race_horses.win_odds, :win_odds),
            place_odds_min = COALESCE(race_horses.place_odds_min, :place_odds_min),
            place_odds_max = COALESCE(race_horses.place_odds_max, :place_odds_max)
    ",
to_params_named(&datum).unwrap().to_slice().as_slice(),
    )
    .unwrap();
}

fn bajikyo_pedigree_to_horses(tx: &Transaction, datum: &Horses) {
    tx.execute(
        "INSERT INTO horses
        (horse_bajikyo_id, sire_bajikyo_id, dam_bajikyo_id, bms_bajikyo_id)
        VALUES (?1, ?2, ?3, ?4)
        ON CONFLICT(horse_bajikyo_id) DO UPDATE SET
        sire_bajikyo_id = COALESCE(?2, horses.sire_bajikyo_id),
        dam_bajikyo_id = COALESCE(?3, horses.dam_bajikyo_id),
        bms_bajikyo_id = COALESCE(?4, horses.bms_bajikyo_id)",
        params![
            datum.horse_bajikyo_id,
            datum.sire_bajikyo_id,
            datum.dam_bajikyo_id,
            datum.bms_bajikyo_id,
        ],
    )
    .unwrap();
}

fn bajikyo_profile_to_horses(tx: &Transaction, datum: &Horses) {
    tx.execute(
        "INSERT INTO horses
        (horse_bajikyo_id, horse_birthdate, horse_birth_year, horse_coat_color, horse_breed, breeder, breeder_location)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        ON CONFLICT(horse_bajikyo_id) DO UPDATE SET
        horse_birthdate = COALESCE(?2, horses.horse_birthdate),
        horse_birth_year = COALESCE(?3, horses.horse_birth_year),
        horse_coat_color = COALESCE(?4, horses.horse_coat_color),
        horse_breed = COALESCE(?5, horses.horse_breed),
        breeder = COALESCE(?6, horses.breeder),
        breeder_location = COALESCE(?7, horses.breeder_location)",
        params![
            datum.horse_bajikyo_id,
            datum.horse_birthdate,
            datum.horse_birth_year,
            datum.horse_coat_color,
            datum.horse_breed,
            datum.breeder,
            datum.breeder_location
        ],
    )
    .unwrap();
}

fn horse_history_to_horses(tx: &Transaction, datum: &Horses) {
    tx.execute(
        "INSERT INTO horses
        (horse_nar_id, horse_name, horse_status, deregistration_date)
        VALUES (?1, ?2, ?3, ?4)
        ON CONFLICT(horse_nar_id) DO UPDATE SET
        horse_name = COALESCE(?2, horses.horse_name),
        horse_status = COALESCE(?3, horses.horse_status),
        deregistration_date = COALESCE(?4, horses.deregistration_date)",
        params![
            datum.horse_nar_id,
            datum.horse_name,
            datum.horse_status,
            datum.deregistration_date,
        ],
    )
    .unwrap();
}

fn horse_profile_to_horses(tx: &Transaction, datum: &Horses) {
    tx.execute(
        "INSERT INTO horses
        (horse_nar_id, horse_bajikyo_id)
        VALUES (?1, ?2)
        ON CONFLICT(horse_nar_id) DO UPDATE SET
        horse_bajikyo_id = COALESCE(?2, horses.horse_bajikyo_id)",
        params![datum.horse_nar_id, datum.horse_bajikyo_id],
    )
    .unwrap();
}

fn jockeys_to_jockeys(tx: &Transaction, datum: &Jockeys) {
    tx.execute(
        "
            INSERT INTO jockeys
            (jockey_nar_id, jockey_name, jockey_kana, jockey_sex, jockey_status,
                jockey_birthdate, jockey_first_run, jockey_first_win)
            VALUES 
            (:jockey_nar_id, :jockey_name, :jockey_kana, :jockey_sex, :jockey_status,
            :jockey_birthdate, :jockey_first_run, :jockey_first_win)
            ON CONFLICT(jockey_nar_id) DO UPDATE SET
            jockey_name = COALESCE(jockeys.jockey_name, :jockey_name),
            jockey_kana = COALESCE(:jockey_kana, jockeys.jockey_kana),
            jockey_sex = COALESCE(:jockey_sex, jockeys.jockey_sex),
            jockey_status = COALESCE(:jockey_status, jockeys.jockey_status),
            jockey_birthdate = COALESCE(:jockey_birthdate, jockeys.jockey_birthdate),
            jockey_first_run = COALESCE(:jockey_first_run, jockeys.jockey_first_run),
            jockey_first_win = COALESCE(:jockey_first_win, jockeys.jockey_first_win)
        ",
        to_params_named(&datum).unwrap().to_slice().as_slice(),
    )
    .unwrap();
}

fn trainers_to_trainers(tx: &Transaction, datum: &Trainers) {
    tx.execute(
        "
            INSERT INTO trainers
            (trainer_nar_id, trainer_name, trainer_kana, trainer_sex, trainer_status,
                trainer_birthdate, trainer_first_run, trainer_first_win)
            VALUES 
            (:trainer_nar_id, :trainer_name, :trainer_kana, :trainer_sex, :trainer_status,
            :trainer_birthdate, :trainer_first_run, :trainer_first_win)
            ON CONFLICT(trainer_nar_id) DO UPDATE SET
            trainer_name = COALESCE(trainers.trainer_name, :trainer_name),
            trainer_kana = COALESCE(:trainer_kana, trainers.trainer_kana),
            trainer_sex = COALESCE(:trainer_sex, trainers.trainer_sex),
            trainer_status = COALESCE(:trainer_status, trainers.trainer_status),
            trainer_birthdate = COALESCE(:trainer_birthdate, trainers.trainer_birthdate),
            trainer_first_run = COALESCE(:trainer_first_run, trainers.trainer_first_run),
            trainer_first_win = COALESCE(:trainer_first_win, trainers.trainer_first_win)
        ",
        to_params_named(&datum).unwrap().to_slice().as_slice(),
    )
    .unwrap();
}
