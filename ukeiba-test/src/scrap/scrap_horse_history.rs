use super::{fetch_and_scrap_all, get_fiscal_year};
use crate::db::{make_conn, Dates, Horses, RaceHorses, Races};
use hashbrown::HashMap;
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
    let jockey_hashmap = create_jockey_hashmap();
    let trainer_hashmap = create_trainer_hashmap();
    for datum in data {
        for x in datum.data {
            dates.push(Dates {
                date: x.race_date,
                racecourse: Some(x.racecourse),
                fiscal_year: get_fiscal_year(x.race_date),
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
                jockey_id: x
                    .jockey_name
                    .and_then(|v| jockey_hashmap.get(&v).map(|&value| value)),
                weight_to_carry: x.weight_to_carry,
                trainer_id: x
                    .trainer_name
                    .and_then(|v| trainer_hashmap.get(&v).map(|&value| value)),
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
            INSERT INTO dates (date, racecourse, fiscal_year, kai, nichi)
            VALUES (:date, :racecourse, :fiscal_year, :kai, :nichi)
            ON CONFLICT(date) DO UPDATE SET
                racecourse = COALESCE(:racecourse, dates.racecourse),
                fiscal_year = COALESCE(:fiscal_year, dates.fiscal_year),
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

fn create_trainer_hashmap() -> HashMap<String, i32> {
    let mut hashmap = HashMap::new();

    hashmap.insert("氏家".to_string(), 18004);
    hashmap.insert("鵜沼武".to_string(), 18005);
    hashmap.insert("太田".to_string(), 18006);
    hashmap.insert("大友".to_string(), 18007);
    hashmap.insert("大友司".to_string(), 18007);
    hashmap.insert("岡田".to_string(), 18009);
    hashmap.insert("岡田定".to_string(), 18009);
    hashmap.insert("尾ケ瀬".to_string(), 18010);
    hashmap.insert("尾瀬富".to_string(), 18010);
    hashmap.insert("片平".to_string(), 18012);
    hashmap.insert("喜来".to_string(), 18013);
    hashmap.insert("久保".to_string(), 18014);
    hashmap.insert("久保正".to_string(), 18014);
    hashmap.insert("小北".to_string(), 18015);
    hashmap.insert("定塚".to_string(), 18016);
    hashmap.insert("鈴木邦".to_string(), 18017);
    hashmap.insert("田上忠".to_string(), 18018);
    hashmap.insert("辻本".to_string(), 18019);
    hashmap.insert("辻本誠".to_string(), 18019);
    hashmap.insert("梨本".to_string(), 18024);
    hashmap.insert("梨本照".to_string(), 18024);
    hashmap.insert("夏井".to_string(), 18025);
    hashmap.insert("西邑".to_string(), 18026);
    hashmap.insert("西邑春".to_string(), 18026);
    hashmap.insert("橋本".to_string(), 18027);
    hashmap.insert("長谷".to_string(), 18028);
    hashmap.insert("服部".to_string(), 18029);
    hashmap.insert("服部義".to_string(), 18029);
    hashmap.insert("林正男".to_string(), 18030);
    hashmap.insert("晴披".to_string(), 18031);
    hashmap.insert("日詰".to_string(), 18032);
    hashmap.insert("渕上".to_string(), 18033);
    hashmap.insert("古田".to_string(), 18034);
    hashmap.insert("古田覺".to_string(), 18034);
    hashmap.insert("前原和".to_string(), 18035);
    hashmap.insert("前原".to_string(), 18035);
    hashmap.insert("前原芳".to_string(), 18036);
    hashmap.insert("松井".to_string(), 18037);
    hashmap.insert("松井浩".to_string(), 18037);
    hashmap.insert("三浦孝".to_string(), 18038);
    hashmap.insert("三浦忠".to_string(), 18039);
    hashmap.insert("水上".to_string(), 18040);
    hashmap.insert("光富".to_string(), 18041);
    hashmap.insert("宮崎正".to_string(), 18042);
    hashmap.insert("山下".to_string(), 18043);
    hashmap.insert("山田".to_string(), 18044);
    hashmap.insert("山田勇".to_string(), 18044);
    hashmap.insert("山本幸".to_string(), 18045);
    hashmap.insert("上山本".to_string(), 18046);
    hashmap.insert("山本俊".to_string(), 18047);
    hashmap.insert("野々宮".to_string(), 18048);
    hashmap.insert("野々豊".to_string(), 18048);
    hashmap.insert("門脇税".to_string(), 18049);
    hashmap.insert("平田義".to_string(), 18050);
    hashmap.insert("福森浩".to_string(), 18051);
    hashmap.insert("小林勝".to_string(), 18052);
    hashmap.insert("林豊".to_string(), 18053);
    hashmap.insert("今井茂".to_string(), 18054);
    hashmap.insert("木村卓".to_string(), 18055);
    hashmap.insert("長部".to_string(), 18056);
    hashmap.insert("長部幸".to_string(), 18056);
    hashmap.insert("大橋".to_string(), 18057);
    hashmap.insert("大橋和".to_string(), 18057);
    hashmap.insert("久田".to_string(), 18058);
    hashmap.insert("久田守".to_string(), 18058);

    hashmap
}

fn create_jockey_hashmap() -> HashMap<String, i32> {
    let mut hashmap = HashMap::new();

    hashmap.insert("荒井幸".to_string(), 38001);
    hashmap.insert("今井茂".to_string(), 38002);
    hashmap.insert("岩瀬和".to_string(), 38003);
    hashmap.insert("岩本利".to_string(), 38004);
    hashmap.insert("岩本正".to_string(), 38005);
    hashmap.insert("大河和".to_string(), 38006);
    hashmap.insert("大友栄".to_string(), 38007);
    hashmap.insert("長部幸".to_string(), 38008);
    hashmap.insert("門脇税".to_string(), 38009);
    hashmap.insert("金山明".to_string(), 38010);
    hashmap.insert("木村".to_string(), 38012);
    hashmap.insert("小林".to_string(), 38013);
    hashmap.insert("坂本東".to_string(), 38014);
    hashmap.insert("鈴木勝".to_string(), 38015);
    hashmap.insert("千葉".to_string(), 38016);
    hashmap.insert("千葉均".to_string(), 38016);
    hashmap.insert("夏井功".to_string(), 38017);
    hashmap.insert("西弘".to_string(), 38018);
    hashmap.insert("西弘美".to_string(), 38018);
    hashmap.insert("西康".to_string(), 38019);
    hashmap.insert("西康幸".to_string(), 38019);
    hashmap.insert("野口哲".to_string(), 38020);
    hashmap.insert("林豊".to_string(), 38021);
    hashmap.insert("久田".to_string(), 38022);
    hashmap.insert("平田".to_string(), 38023);
    hashmap.insert("藤野俊".to_string(), 38024);
    hashmap.insert("藤本匠".to_string(), 38025);
    hashmap.insert("前原陽".to_string(), 38026);
    hashmap.insert("松井浩".to_string(), 38027);
    hashmap.insert("皆川公".to_string(), 38028);
    hashmap.insert("宮本直".to_string(), 38029);
    hashmap.insert("森芳浩".to_string(), 38030);
    hashmap.insert("山本正".to_string(), 38031);
    hashmap.insert("松田道".to_string(), 38033);
    hashmap.insert("大口泰".to_string(), 38034);
    hashmap.insert("千田輝".to_string(), 38035);
    hashmap.insert("細川弘".to_string(), 38036);
    hashmap.insert("尾瀬馨".to_string(), 38037);
    hashmap.insert("工藤篤".to_string(), 38038);
    hashmap.insert("安部憲".to_string(), 38039);
    hashmap.insert("金田勇".to_string(), 38040);
    hashmap.insert("折口秀".to_string(), 38041);
    hashmap.insert("佐渡誠".to_string(), 38042);
    hashmap.insert("古谷輝".to_string(), 38043);
    hashmap.insert("村上慎".to_string(), 38044);
    hashmap.insert("加藤修".to_string(), 38045);
    hashmap.insert("松本和".to_string(), 38046);
    hashmap.insert("村上章".to_string(), 38047);
    hashmap.insert("澁谷益".to_string(), 38048);
    hashmap.insert("辻本由".to_string(), 38049);
    hashmap.insert("綱村裕".to_string(), 38050);
    hashmap.insert("渡辺".to_string(), 38051);

    hashmap
}
