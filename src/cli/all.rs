use crate::storage::Storage;
use clap::ArgMatches;

/// Adds kanji to the storage
pub fn run(storage: Storage, app: &ArgMatches) {
    let separated = !app.is_present("raw");

    for (pos, kanji) in storage.iter().map(|i| i.get_literal()).enumerate() {
        if pos > 0 && separated {
            print!(",");
        }

        print!("{kanji}");
    }

    println!();
}
