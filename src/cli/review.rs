use crate::{japanese::JapaneseExt, sm2::RepQuality, storage::Storage};
use clap::ArgMatches;

use super::{add::parse_kanji_arg, run::read_failed_input};

/// Manually mark kanji as reviewed
pub fn run(mut storage: Storage, app: &ArgMatches) {
    let inp = match parse_kanji_arg(app) {
        Some(inp) => inp,
        None => return,
    };

    let inp_kanji = inp
        .chars()
        .filter(|i| i.is_kanji())
        .filter_map(|i| {
            let item = storage.get_by_lit(i)?;
            if !item.can_be_reviewed() {
                return None;
            }

            Some(item)
        })
        .collect::<Vec<_>>();

    if inp_kanji.is_empty() {
        println!("Nothing to do");
        return;
    }

    let to_relearn = read_failed_input()
        .into_iter()
        .filter(|i| inp_kanji.iter().any(|j| j.get_literal() == *i))
        .collect::<Vec<_>>();

    let mut reviewd = vec![];

    for (id, lit) in inp_kanji
        .into_iter()
        .map(|i| (i.get_id(), i.get_literal()))
        .collect::<Vec<_>>()
    {
        let item_mut = storage.get_srs_mut(id).unwrap();

        if to_relearn.contains(&lit) {
            item_mut.review(RepQuality::Grade2)
        } else {
            item_mut.review(RepQuality::Grade4)
        }
        reviewd.push(lit.to_string());
    }

    if !reviewd.is_empty() {
        println!("Reviewed {}", reviewd.join(","));
    } else {
        println!("Nothing to reset");
    }
}
