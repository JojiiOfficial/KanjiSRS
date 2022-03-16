use crate::{japanese::JapaneseExt, storage::Storage, utils};
use clap::ArgMatches;

/// Adds kanji to the storage
pub fn run(mut storage: Storage, app: &ArgMatches) {
    let inp = match parse_kanji_arg(app) {
        Some(inp) => inp,
        None => return,
    };

    let mut added = vec![];

    for kanji in inp.chars().filter(|i| i.is_kanji()) {
        if storage.add(kanji) {
            added.push(kanji.to_string());
        }
    }

    if !added.is_empty() {
        println!("Added {}", added.join(","));
    } else {
        println!("Nothing to add");
    }
}

pub(crate) fn parse_kanji_arg(app: &ArgMatches) -> Option<String> {
    let val = match app.value_of("kanji") {
        Some(v) => v,
        None => {
            println!("Missing kanji!");
            return None;
        }
    };

    if val == "-" {
        Some(utils::read_stdin())
    } else {
        if !val.has_kanji() {
            println!("Missing kanji!");
            return None;
        }
        Some(val.to_string())
    }
}
