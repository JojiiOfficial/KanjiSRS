use crate::{cli::run::DEFAULT_NEW, storage::Storage};
use clap::ArgMatches;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};

/// Show ovarall stats
pub fn run(storage: Storage, _app: &ArgMatches) {
    let mut table = Table::new();

    table.max_column_width = 30;
    table.style = TableStyle::rounded();

    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        "Kanji stats",
        2,
        Alignment::Center,
    )]));

    table.add_row(Row::new(vec![
        TableCell::new("Total Kanji"),
        TableCell::new_with_alignment(format!("{}字", storage.len()), 1, Alignment::Left),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("In learning"),
        TableCell::new_with_alignment(
            format!("{}字", storage.learning_kanji()),
            1,
            Alignment::Left,
        ),
    ]));

    let percent = (storage.learning_kanji() as f32) * 100.0 / storage.len() as f32;
    table.add_row(Row::new(vec![
        TableCell::new("Percentage"),
        TableCell::new_with_alignment(format!("{percent:.1}%"), 1, Alignment::Left),
    ]));

    let left = storage.len() - storage.learning_kanji();
    let days_left = (left as f32 / DEFAULT_NEW as f32).ceil() as usize;
    table.add_row(Row::new(vec![
        TableCell::new("Days left"),
        TableCell::new_with_alignment(format!("{}日", days_left), 1, Alignment::Left),
    ]));

    println!("{}", table.render());
}
