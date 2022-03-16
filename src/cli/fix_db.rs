use crate::{storage::Storage, utils};
use clap::ArgMatches;

/// Fix database
pub fn run(mut storage: Storage, _app: &ArgMatches) {
    if !storage.check() {
        println!("Database broken");
        let confirmation = utils::confirmation("Do you want to repair it?");
        if !confirmation {
            return;
        }
    }

    println!("Trying to repair database");

    // Repair database
    let success = storage.repair();

    if success {
        println!("Success");
    } else {
        println!("Couldn't fix database");
    }
}
