use crate::common::*;
use rayon::prelude::*;
use ukeiba_common::{common::HorseBelong, scraper::jockey_search};

pub fn get_all_from_nar() -> Vec<i32> {
    let pages: Vec<jockey_search::Page> = [jockey_search::Page {
        page_num: 1,
        belong: HorseBelong::Banei,
    }]
    .to_vec();

    let search_pages: Vec<jockey_search::Page> = fetch_and_scrap_all(pages)
        .par_iter()
        .map(|page| {
            let hits = page.hits;
            if hits == 0 {
                return Vec::new();
            }
            let page_count = (hits - 1) / 50 + 1;
            (1..=page_count)
                .map(|page_num| jockey_search::Page {
                    page_num: page_num,
                    belong: page.belong,
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>()
        .concat();

    let jockey_all_ids = fetch_and_scrap_all(search_pages)
        .into_iter()
        .flat_map(|data| data.jockey_ids)
        .collect::<Vec<_>>();

    jockey_all_ids
}
