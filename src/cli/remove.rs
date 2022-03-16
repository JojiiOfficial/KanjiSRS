use super::add::parse_kanji_arg;
use crate::{japanese::JapaneseExt, storage::Storage};
use clap::ArgMatches;

/// Removes kanji from the storage
pub fn run(mut storage: Storage, app: &ArgMatches) {
    let inp = match parse_kanji_arg(app) {
        Some(inp) => inp,
        None => return,
    };

    let mut removed = vec![];

    for kanji in inp.chars().filter(|i| i.is_kanji()) {
        if storage.remove(kanji) {
            removed.push(kanji.to_string());
        }
    }

    if !removed.is_empty() {
        println!("Removed {}", removed.join(","));
    } else {
        println!("Nothing to remove");
    }
}
