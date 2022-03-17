use crate::{
    browser,
    japanese::JapaneseExt,
    sm2::RepQuality,
    storage::{Item, Storage},
    utils,
};
use clap::ArgMatches;

pub(crate) const DEFAULT_NEW: usize = 8;
pub(crate) const MAX_REVIEWS: usize = 20;

pub struct RunConfigs {
    new_count: usize,
    max_reviews: usize,
}

pub fn run(mut storage: Storage, app: ArgMatches) {
    let run_config = parse_runconfig(&app);

    let to_learn = pick_to_learn(&storage, &run_config);
    if to_learn.is_empty() {
        println!("Nothing to learn nor review. Try adding some new kanji");
        return;
    }

    if !utils::confirmation("Do you want to start a review?") {
        return;
    }

    let has_reviews = to_learn.iter().any(|i| i.is_learning());
    if !has_reviews && !utils::confirmation("No reviews available. Learn more?") {
        return;
    }

    let learn_string = to_learn.iter().map(|i| i.get_literal()).collect::<String>();
    browser::open_kanji(&learn_string);

    let to_relearn = determine_relearn(&storage, &to_learn);

    for i in to_learn.into_iter().map(|i| i.get_id()).collect::<Vec<_>>() {
        let item_mut = storage.get_srs_mut(i).unwrap();

        if to_relearn.contains(&i) {
            item_mut.review(RepQuality::Grade2)
        } else {
            item_mut.review(RepQuality::Grade4)
        }
    }

    println!("Learning done");
}

pub fn read_failed_input() -> Vec<char> {
    utils::print_stdout("Enter kanji you want to learn again > ");
    utils::read_std_line()
        .chars()
        .filter(|i| i.is_kanji())
        .collect::<Vec<char>>()
}

fn determine_relearn(storage: &Storage, learned: &[Item<'_>]) -> Vec<u32> {
    read_failed_input()
        .into_iter()
        .filter_map(|r| {
            learned
                .iter()
                .any(|i| i.get_literal() == r)
                .then(|| storage.get_by_lit(r))
                .flatten()
        })
        .map(|i| i.get_id())
        .collect::<Vec<_>>()
}

fn pick_to_learn<'a>(storage: &'a Storage, run_config: &RunConfigs) -> Vec<Item<'a>> {
    let due = storage.get_srs_storage().get_due().map(|i| i.id);
    let reviews = if run_config.max_reviews == 0 {
        // take all
        due.collect::<Vec<_>>()
    } else {
        // take as much as user wanted to review
        due.take(run_config.max_reviews).collect::<Vec<_>>()
    };

    reviews
        .into_iter()
        .chain(
            // add new reviews
            storage
                .get_srs_storage()
                .get_new()
                .map(|i| i.id)
                .take(run_config.new_count),
        )
        .filter_map(|i| storage.get_by_id(i))
        .collect::<Vec<_>>()
}

fn parse_runconfig(app: &ArgMatches) -> RunConfigs {
    let new_count = if !app.is_present("no-new") {
        utils::parse_nr(app.value_of("new-count"), DEFAULT_NEW)
    } else {
        0
    };

    let max_reviews = utils::parse_nr(app.value_of("max-reviews"), MAX_REVIEWS);

    RunConfigs {
        new_count,
        max_reviews,
    }
}
