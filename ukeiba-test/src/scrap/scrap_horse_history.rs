use super::fetch_and_scrap_all;
use crate::db::{make_conn, Dates, Horses, RaceHorses, Races};
use rusqlite::params;
use serde_rusqlite::to_params_named;
use ukeiba_common::scraper::horse_history;

pub fn scrap() {
    let conn = make_conn().unwrap();

    // horse_bajikyo_idを取得するクエリ
    let query = "SELECT horse_nar_id FROM horses";

    // クエリを実行し、結果を取得
    let mut stmt = conn.prepare(query).unwrap();
    let rows = stmt.query_map([], |row| row.get(0)).unwrap();

    // horse_nar_ids<String>に格納
    let horse_nar_ids: Vec<i64> = rows.map(|row| row.unwrap()).collect();

    let pages = horse_nar_ids
        .iter()
        .map(|x| horse_history::Page {
            horse_nar_id: x.clone(),
        })
        .collect::<Vec<_>>();

    let data = fetch_and_scrap_all(pages);
    let mut dates: Vec<Dates> = Vec::new();
    let mut races: Vec<Races> = Vec::new();
    let mut race_horses: Vec<RaceHorses> = Vec::new();
    let mut horse_data: Vec<Horses> = Vec::new();
    for datum in data {
        for x in datum.data {
            dates.push(Dates {
                date: x.race_date,
                racecourse: Some(x.racecourse),
                ..Default::default()
            });
            races.push(Races {
                date: x.race_date,
                race_num: x.race_num,
                race_type: x.race_type,
                weather: x.weather,
                going: x.going.and_then(|x| x.parse().ok()),
                horse_count: x.horse_count,
                race_name: x.race_name,
                ..Default::default()
            });
            race_horses.push(RaceHorses {
                date: x.race_date,
                race_num: x.race_num,
                horse_num: x.horse_num.unwrap_or_default(),
                horse_nar_id: Some(x.horse_nar_id),
                bracket_num: x.bracket_num,
                win_fav: x.win_fav,
                horse_weight: x.horse_weight,
                jockey_id: None, //仮
                weight_to_carry: x.weight_to_carry,
                trainer_id: None,
                arrival: x.arrival,
                arrival_info: x.arrival_raw,
                finish_time: x.finish_time,
                prize: x.prize,
                ..Default::default()
            })
        }
        horse_data.push(Horses {
            horse_nar_id: Some(datum.horse_nar_id),
            horse_name: Some(datum.horse_name),
            horse_status: Some(datum.horse_status),
            deregistration_date: datum.deregistration_date,
            ..Default::default()
        });
    }

    let mut conn = make_conn().unwrap();
    let tx = conn.transaction().unwrap();
    for datum in dates {
        tx.execute(
            "
            INSERT INTO dates (date, racecourse, kai, nichi)
            VALUES (:date, :racecourse, :kai, :nichi)
            ON CONFLICT(date) DO UPDATE SET
                racecourse = COALESCE(:racecourse, dates.racecourse),
                kai = COALESCE(:kai, dates.kai),
                nichi = COALESCE(:nichi, dates.nichi)
        ",
            to_params_named(&datum).unwrap().to_slice().as_slice(),
        )
        .unwrap();
    }

    for datum in races {
        tx.execute(
            "
            INSERT INTO races 
            (date, race_num, race_type, weather, going,
            horse_count, post_time, post_time_change, race_sub_name, race_name,
            race_weight_type)
            VALUES (:date, :race_num, :race_type, :weather, :going,
            :horse_count, :post_time, :post_time_change, :race_sub_name, :race_name,
            :race_weight_type)
            ON CONFLICT(date, race_num) DO UPDATE SET
            race_type = COALESCE(:race_type, races.race_type),
            weather = COALESCE(:weather, races.weather),
            going = COALESCE(:going, races.going),
            horse_count = COALESCE(:horse_count, races.horse_count),
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

    for datum in race_horses {
        tx.execute(
            "
            INSERT INTO race_horses 
            (date, race_num, horse_num, horse_nar_id, bracket_num,
                win_fav, horse_weight, jockey_id, weight_to_carry, trainer_id,
                arrival, arrival_info, finish_time, prize, change,
                horse_sex, weight_mark, owner_name, win_odds, place_odds_min,
                place_odds_max)
            VALUES (:date, :race_num, :horse_num, :horse_nar_id, :bracket_num,
                :win_fav, :horse_weight, :jockey_id, :weight_to_carry, :trainer_id,
                :arrival, :arrival_info, :finish_time, :prize, :change,
                :horse_sex, :weight_mark, :owner_name, :win_odds, :place_odds_min,
                :place_odds_max)
            ON CONFLICT(date, race_num, horse_num) DO UPDATE SET
            horse_nar_id = COALESCE(:horse_nar_id, race_horses.horse_nar_id),
            bracket_num = COALESCE(:bracket_num, race_horses.bracket_num),

            win_fav = COALESCE(:win_fav, race_horses.win_fav),
            horse_weight = COALESCE(:horse_weight, race_horses.horse_weight),
            jockey_id = COALESCE(:jockey_id, race_horses.jockey_id),
            weight_to_carry = COALESCE(:weight_to_carry, race_horses.weight_to_carry),
            trainer_id = COALESCE(:trainer_id, race_horses.trainer_id),

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

    for horse_datum in horse_data {
        tx.execute(
            "INSERT INTO horses
            (horse_nar_id, horse_name, horse_status, deregistration_date)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(horse_nar_id) DO UPDATE SET
            horse_name = COALESCE(?2, horses.horse_name),
            horse_status = COALESCE(?3, horses.horse_status),
            deregistration_date = COALESCE(?4, horses.deregistration_date)",
            params![
                horse_datum.horse_nar_id,
                horse_datum.horse_name,
                horse_datum.horse_status,
                horse_datum.deregistration_date,
            ],
        )
        .unwrap();
    }
    tx.commit().unwrap();

    // horse_bajikyo_idsを利用する
}
