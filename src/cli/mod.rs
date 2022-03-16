pub mod add;
pub mod fix_db;
pub mod info;
pub mod remove;
pub mod reset;
pub mod review;
pub mod run;
pub mod stats;

pub use run::run;

use clap::{App, AppSettings, Arg};

pub fn build() -> App<'static> {
    let app = App::new("ksrs")
        .about("Tool to help learning kanji")
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::new("no-new")
                .help("Don't add new kanji, just review old")
                .long("no-new"),
        )
        .arg(
            Arg::new("max-reviews")
                .help("Max amount of reviews")
                .takes_value(true)
                .aliases(&["max-review"])
                .long("max-reviews"),
        )
        .arg(
            Arg::new("new-count")
                .help("Specify how many new cards you want to learn")
                .long("new-count"),
        )
        .subcommand(
            App::new("add")
                .about("Adds kanji to learn")
                .arg(Arg::new("kanji")),
        )
        .subcommand(
            App::new("remove")
                .about("Removes kanji from database")
                .arg(Arg::new("kanji")),
        )
        .subcommand(App::new("info").about("Show info about reviews"))
        .subcommand(
            App::new("reset")
                .about("Reset learn process of a kanji and treat it as a new item")
                .arg(Arg::new("kanji")),
        )
        .subcommand(
            App::new("review")
                .about("Manually tag kanji as reviewed")
                .arg(Arg::new("kanji")),
        )
        .subcommand(App::new("fix-db").about("Fix database"))
        .subcommand(App::new("stats").about("Show stats"));

    app
}
