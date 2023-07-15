use crate::common::*;
use rayon::prelude::*;
use ukeiba_common::{
    common::{HorseBelong, TrainerNarId},
    scraper::trainer_search,
};

pub fn get_all_from_nar() -> Vec<TrainerNarId> {
    let pages: Vec<trainer_search::Page> = [trainer_search::Page {
        page_num: 1,
        belong: HorseBelong::Banei,
    }]
    .to_vec();

    let search_pages: Vec<trainer_search::Page> = fetch_and_scrap_all(pages)
        .par_iter()
        .map(|page| {
            let hits = page.hits;
            if hits == 0 {
                return Vec::new();
            }
            let page_count = (hits - 1) / 50 + 1;
            (1..=page_count)
                .map(|page_num| trainer_search::Page {
                    page_num: page_num,
                    belong: page.belong,
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>()
        .concat();

    let trainer_all_ids = fetch_and_scrap_all(search_pages)
        .into_iter()
        .flat_map(|data| data.trainer_ids)
        .map(|x| TrainerNarId(x))
        .collect::<Vec<_>>();

    trainer_all_ids
}
