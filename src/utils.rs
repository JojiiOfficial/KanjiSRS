use chrono::{Duration, Timelike};
use std::io::{Read, Write};

pub fn read_stdin() -> String {
    let mut buf = vec![];
    std::io::stdin().read_to_end(&mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

/// Returns the unix timestamp of 04:00 of the current day
pub fn get_today_unix() -> u64 {
    let 今 = chrono::Utc::now().with_hour(4).unwrap();
    let 今 = 今.with_second(0).unwrap();
    let 今 = 今.with_minute(0).unwrap();
    今.timestamp() as u64
}

pub fn unix_n_days_offset(days: u32) -> u64 {
    let mut 今 = chrono::Utc::now();
    今 = 今 + Duration::days(days as i64);
    今.with_second(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_hour(4)
        .unwrap()
        .timestamp() as u64
}

pub fn print_stdout(text: &str) {
    print!("{text}");
    std::io::stdout().flush().unwrap();
}

pub(crate) fn read_std_line() -> String {
    let mut out = String::new();
    std::io::stdin().read_line(&mut out).unwrap();
    out
}

/// Does a user confirmation with `text` and returns `true` if user agreed
pub fn confirmation(text: &str) -> bool {
    print_stdout(&format!("{text} (yes/no)> "));
    let inp = read_std_line().to_lowercase().trim().replace("\n", "");
    ["yes", "y", "ja", "はい", ""].contains(&inp.as_str())
}

pub fn parse_nr(s: Option<&str>, default: usize) -> usize {
    s.and_then(|i| {
        let n: usize = i.parse().ok()?;
        Some(n)
    })
    .unwrap_or(default)
}
