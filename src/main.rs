pub mod browser;
pub mod cli;
pub mod japanese;
pub mod sm2;
pub mod storage;
pub mod utils;

use std::{path::PathBuf, str::FromStr};

use proc_lock::{lock, LockPath};

use crate::storage::Storage;

fn get_storage_path() -> PathBuf {
    let path = PathBuf::from_str("./storage/").unwrap();
    if !path.exists() {
        std::fs::create_dir(&path).unwrap();
    }

    path
}

fn main() {
    let app = cli::build().get_matches();
    let path = get_storage_path();
    let guard = lock(&LockPath::Tmp("kanji_srs.lock")).unwrap();
    let item_storage = path.join("item_storage").to_str().unwrap().to_string();
    let srs_storage = path.join("srs_storage").to_str().unwrap().to_string();
    let item_storage = std::thread::spawn(move || storage::ItemStorage::new(item_storage));
    let srs_storage = std::thread::spawn(move || storage::SRSStorage::new(srs_storage));
    let srs_storage = srs_storage.join().unwrap();
    let item_storage = item_storage.join().unwrap();
    let storage = Storage::new(item_storage, srs_storage);

    let sub_command = app.subcommand();

    if !storage.check() {
        if let Some(("fix-db", _)) = sub_command.as_ref() {
        } else {
            println!("Database broken. Run with --fix-db to fix it");
            return;
        }
    }

    // println!("{:#?}", storage);

    /*
    for i in storage.get_srs_storage_mut().iter_mut() {
        i.直す();
    }
    */

    match sub_command {
        Some(("add", sub_matches)) => cli::add::run(storage, sub_matches),
        Some(("remove", sub_matches)) => cli::remove::run(storage, sub_matches),
        Some(("reset", sub_matches)) => cli::reset::run(storage, sub_matches),
        Some(("info", sub_matches)) => cli::info::run(storage, sub_matches),
        Some(("review", sub_matches)) => cli::review::run(storage, sub_matches),
        Some(("fix-db", sub_matches)) => cli::fix_db::run(storage, sub_matches),
        Some(("stats", sub_matches)) => cli::stats::run(storage, sub_matches),
        Some(("all", sub_matches)) => cli::all::run(storage, sub_matches),
        _ => cli::run(storage, app),
    }
    drop(guard);
}
