use crate::common::*;
use crate::db::make_conn;
use itertools::iproduct;
use rayon::prelude::*;
use ukeiba_common::{common::HorseBelong, scraper::horse_search};

pub fn get_all_from_db() -> Vec<i64> {
    let conn = make_conn().unwrap();

    // horse_bajikyo_idを取得するクエリ
    let query = "SELECT horse_nar_id FROM horses";

    // クエリを実行し、結果を取得
    let mut stmt = conn.prepare(query).unwrap();
    let rows = stmt.query_map([], |row| row.get(0)).unwrap();

    // horse_nar_ids<String>に格納
    let horse_nar_ids: Vec<i64> = rows.map(|row| row.unwrap()).collect();
    horse_nar_ids
}

pub fn get_all_from_nar() -> Vec<i64> {
    // 所属がばんえいか退厩の馬を全取得

    let pages: Vec<horse_search::Page> =
        iproduct!([HorseBelong::Banei, HorseBelong::Left], (1976..=2021).rev())
            .map(|(belong, year)| horse_search::Page {
                page_num: 1,
                horse_name: "".to_string(),
                horse_belong: belong,
                birth_year: year,
            })
            .collect();

    let search_pages: Vec<horse_search::Page> = fetch_and_scrap_all(pages)
        .par_iter()
        .map(|page| {
            let hits = page.hits_all;
            match hits {
                0 => Vec::new(),
                0..=2000 => [horse_search::Page {
                    page_num: 1,
                    horse_name: "".to_string(),
                    horse_belong: page.horse_belong,
                    birth_year: page.birth_year,
                }]
                .to_vec(),
                _ => "アイウエオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモヤユヨラリルレロワヲンヴ"
                    .chars()
                    .map(|kana| horse_search::Page {
                        page_num: 1,
                        horse_name: kana.to_string(),
                        horse_belong: page.horse_belong,
                        birth_year: page.birth_year,
                    })
                    .collect::<Vec<_>>(),
            }
        })
        .collect::<Vec<Vec<_>>>()
        .concat();

    let search_pages: Vec<horse_search::Page> = fetch_and_scrap_all(search_pages)
        .par_iter()
        .map(|page| {
            let hits = page.hits_all;
            if hits == 0 {
                return Vec::new();
            }
            let page_count = (hits - 1) / 50 + 1;
            (1..=page_count)
                .map(|page_num| horse_search::Page {
                    page_num: page_num,
                    horse_name: page.horse_name.clone(),
                    horse_belong: page.horse_belong,
                    birth_year: page.birth_year,
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>()
        .concat();

    let horse_all_ids = fetch_and_scrap_all(search_pages)
        .into_iter()
        .flat_map(|data| data.horse_nar_ids)
        .collect::<Vec<_>>();

    horse_all_ids
}
