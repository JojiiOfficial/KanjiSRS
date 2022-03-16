use super::add::parse_kanji_arg;
use crate::{japanese::JapaneseExt, storage::Storage};
use clap::ArgMatches;

/// Reset kanji
pub fn run(mut storage: Storage, app: &ArgMatches) {
    let inp = match parse_kanji_arg(app) {
        Some(inp) => inp,
        None => return,
    };

    let mut reset = vec![];

    for kanji in inp.chars().filter(|i| i.is_kanji()) {
        if storage.reset(kanji) {
            reset.push(kanji.to_string());
        }
    }

    if !reset.is_empty() {
        println!("Resetted {}", reset.join(","));
    } else {
        println!("Nothing to reset");
    }
}
