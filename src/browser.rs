/// Search type kanji
const KANJI_SEARCH_TYPE: u8 = 1;

/// Returns a jotoba-search url with the given query and search type
pub fn get_jotoba_url(query: &str, s_type: u8) -> String {
    format!("https://jotoba.de/search/{query}?t={s_type}")
}

/// Opens jotoba kanji page with given kanji as query
pub fn open_kanji(kanji: &str) {
    open::that(get_jotoba_url(kanji, KANJI_SEARCH_TYPE)).unwrap();
}
