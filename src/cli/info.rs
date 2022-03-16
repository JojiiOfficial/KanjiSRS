use crate::{
    storage::{srs, Storage},
    utils,
};
use clap::ArgMatches;

/// Show info about reviews
pub fn run(storage: Storage, _app: &ArgMatches) {
    let mut next = storage.get_srs_storage().get_new().collect::<Vec<_>>();
    if !next.is_empty() {
        next.sort_by(|a, b| a.id.cmp(&b.id));
        println!("Next: ");
        print_review_day(
            &storage,
            &next.into_iter().map(|i| i.id).collect::<Vec<_>>(),
        );
        println!();
    }

    let today = storage.get_srs_storage().get_due().collect::<Vec<_>>();
    if !today.is_empty() {
        let s = today
            .into_iter()
            .filter_map(|i| storage.get_by_id(i.id))
            .map(|i| i.get_literal().to_string())
            .collect::<Vec<_>>()
            .join(",");
        println!("Today: {s}");
        println!();
    }

    let tomorrow = reviews_tomorrow(&storage);
    if !tomorrow.is_empty() {
        println!("Tomorrow: ");
        print_review_day(&storage, &tomorrow);
        println!();
    }

    let tomorrow_time = utils::unix_n_days_offset(1);
    let mut future = all_due_ordered(&storage)
        .into_iter()
        .filter(|i| i.due_on as u64 > tomorrow_time)
        .collect::<Vec<_>>();
    future.sort_by(|a, b| a.id.cmp(&b.id));
    future.sort_by(|a, b| a.due_on.cmp(&b.due_on));
    future.truncate(20);

    if !future.is_empty() {
        println!("Future: ");
        print_review_day(
            &storage,
            &future.into_iter().map(|i| i.id).collect::<Vec<_>>(),
        );
    }

    if storage.is_empty() {
        println!("No kanji in database. Go and add some");
    }
}

pub fn print_review_day(storage: &Storage, data: &[u32]) {
    if data.is_empty() {
        return;
    }

    let mut data = data.to_vec();
    data.sort();

    let items = data
        .iter()
        .filter_map(|i| storage.get_by_id(*i))
        .take(41)
        .collect::<Vec<_>>();
    let more = items.len() > 40;

    let mut items = items
        .into_iter()
        .map(|i| i.get_literal().to_string())
        .collect::<Vec<_>>();
    items.truncate(40);
    let mut s: String = items.join(",");
    if more {
        s.push_str(",...");
    }
    println!("{s}");
}

pub fn reviews_tomorrow(storage: &Storage) -> Vec<u32> {
    let 明日 = utils::unix_n_days_offset(1);
    let 明後日 = utils::unix_n_days_offset(2);
    storage
        .get_srs_storage()
        .iter()
        .filter(|i| i.due_on >= 明日 && i.in_learning && i.due_on < 明後日)
        .map(|i| i.id)
        .collect()
}

pub fn all_due_ordered(storage: &Storage) -> Vec<&srs::Item> {
    let mut srs_items = storage.get_srs_storage().iter().collect::<Vec<_>>();
    srs_items.sort_by(|a, b| a.due_on.cmp(&b.due_on));
    srs_items
}
