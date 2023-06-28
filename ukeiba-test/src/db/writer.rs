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
    HorseHistoryToDates(Dates),
    HorseHistoryToRaces(Races),
    HorseHistoryToRaceHorses(RaceHorses),
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
            Self::HorseHistoryToDates(datum) => horse_history_to_dates(tx, &datum),
            Self::HorseHistoryToRaces(datum) => horse_history_to_races(tx, &datum),
            Self::HorseHistoryToRaceHorses(datum) => horse_history_to_race_horses(tx, &datum),
            Self::BajikyoPedigreeToHorses(datum) => bajikyo_pedigree_to_horses(tx, &datum),
            Self::BajikyoProfileToHorses(datum) => bajikyo_profile_to_horses(tx, &datum),
            Self::HorseHistoryToHorses(datum) => horse_history_to_horses(tx, &datum),
            Self::HorseProfileToHorses(datum) => horse_profile_to_horses(tx, &datum),
            Self::JockeysToJockeys(datum) => jockeys_to_jockeys(tx, &datum),
            Self::TrainersToTrainers(datum) => trainers_to_trainers(tx, &datum),
        }
    }
}

fn horse_history_to_dates(tx: &Transaction, datum: &Dates) {
    tx.execute(
        "
        INSERT INTO dates (race_date, racecourse, fiscal_year, kai, nichi)
        VALUES (:race_date, :racecourse, :fiscal_year, :kai, :nichi)
        ON CONFLICT(race_date) DO UPDATE SET
            racecourse = COALESCE(:racecourse, dates.racecourse),
            fiscal_year = COALESCE(:fiscal_year, dates.fiscal_year),
            kai = COALESCE(:kai, dates.kai),
            nichi = COALESCE(:nichi, dates.nichi)
    ",
        to_params_named(&datum).unwrap().to_slice().as_slice(),
    )
    .unwrap();
}

fn horse_history_to_races(tx: &Transaction, datum: &Races) {
    tx.execute(
        "
        INSERT INTO races 
        (race_date, race_num, race_type, weather, going,
        horse_count_run, post_time, post_time_change, race_sub_name, race_name,
        race_weight_type)
        VALUES (:race_date, :race_num, :race_type, :weather, :going,
        :horse_count_run, :post_time, :post_time_change, :race_sub_name, :race_name,
        :race_weight_type)
        ON CONFLICT(race_date, race_num) DO UPDATE SET
        race_type = COALESCE(:race_type, races.race_type),
        weather = COALESCE(:weather, races.weather),
        going = COALESCE(:going, races.going),
        horse_count_run = COALESCE(:horse_count_run, races.horse_count_run),
        post_time = COALESCE(races.post_time, :post_time),
        post_time_change = COALESCE(races.post_time_change, :post_time_change),
        race_sub_name = COALESCE(races.race_sub_name, :race_sub_name),
        race_name = COALESCE(races.race_name, :race_name),
        race_weight_type = COALESCE(races.race_weight_type, :race_weight_type)
    ",
        to_params_named(&datum).unwrap().to_slice().as_slice(),
    )
    .unwrap();
}

fn horse_history_to_race_horses(tx: &Transaction, datum: &RaceHorses) {
    tx.execute(
        "
        INSERT INTO race_horses 
        (race_date, race_num, horse_num, horse_nar_id, bracket_num,
            win_fav, horse_weight, jockey_nar_id, weight_to_carry, trainer_nar_id,
            arrival, arrival_info, finish_time, prize, change,
            horse_sex, weight_mark, owner_name, win_odds, place_odds_min,
            place_odds_max)
        VALUES (:race_date, :race_num, :horse_num, :horse_nar_id, :bracket_num,
            :win_fav, :horse_weight, :jockey_nar_id, :weight_to_carry, :trainer_nar_id,
            :arrival, :arrival_info, :finish_time, :prize, :change,
            :horse_sex, :weight_mark, :owner_name, :win_odds, :place_odds_min,
            :place_odds_max)
        ON CONFLICT(race_date, race_num, horse_num) DO UPDATE SET
        horse_nar_id = COALESCE(:horse_nar_id, race_horses.horse_nar_id),
        bracket_num = COALESCE(:bracket_num, race_horses.bracket_num),

        win_fav = COALESCE(:win_fav, race_horses.win_fav),
        horse_weight = COALESCE(:horse_weight, race_horses.horse_weight),
        jockey_nar_id = COALESCE(:jockey_nar_id, race_horses.jockey_nar_id),
        weight_to_carry = COALESCE(:weight_to_carry, race_horses.weight_to_carry),
        trainer_nar_id = COALESCE(:trainer_nar_id, race_horses.trainer_nar_id),

        arrival = COALESCE(:arrival, race_horses.arrival),
        arrival_info = COALESCE(:arrival_info, race_horses.arrival_info),
        finish_time = COALESCE(:finish_time, race_horses.finish_time),
        prize = COALESCE(:prize, race_horses.prize),
        change = COALESCE(race_horses.change, :change),

        horse_sex = COALESCE(race_horses.horse_sex, :horse_sex),
        weight_mark = COALESCE(race_horses.weight_mark, :weight_mark),
        owner_name = COALESCE(race_horses.owner_name, :owner_name),
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
        (horse_bajikyo_id, horse_birthdate, horse_coat_color, horse_breed, breeder, breeder_location)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        ON CONFLICT(horse_bajikyo_id) DO UPDATE SET
        horse_birthdate = COALESCE(?2, horses.horse_birthdate),
        horse_coat_color = COALESCE(?3, horses.horse_coat_color),
        horse_breed = COALESCE(?4, horses.horse_breed),
        breeder = COALESCE(?5, horses.breeder),
        breeder_location = COALESCE(?6, horses.breeder_location)",
        params![
            datum.horse_bajikyo_id,
            datum.horse_birthdate,
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
