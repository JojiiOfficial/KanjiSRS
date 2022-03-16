pub trait JapaneseExt {
    /// Returns true if self contains at least one kana character
    fn has_kana(&self) -> bool;

    /// Returns true if self is entirely written in kana
    fn is_kana(&self) -> bool;

    /// Returns true if inp is entirely written with kanji
    fn is_kanji(&self) -> bool;

    /// Returns true if inp has at least one kanji
    fn has_kanji(&self) -> bool;

    /// Returns true if inp is build with kanji and kana only
    fn is_japanese(&self) -> bool;

    /// Returns true if inp contains japanese characters
    fn has_japanese(&self) -> bool;

    /// Returns true if self is written in katakana
    fn is_katakana(&self) -> bool;

    /// Returns true if self is written in hiragana
    fn is_hiragana(&self) -> bool;

    /// Returns the amount of kanji self has
    fn kanji_count(&self) -> usize;

    /// Returns true if self is a (cjk) symbol
    fn is_symbol(&self) -> bool;

    /// Returns true if self is a (cjk) symbol
    fn has_symbol(&self) -> bool;

    fn has_roman_letter(&self) -> bool;

    fn is_roman_letter(&self) -> bool;

    /// Returns true if self is a small katakana letter
    fn is_small_katakana(&self) -> bool;

    /// Returns true if self is a small hiragana letter
    fn is_small_hiragana(&self) -> bool;

    /// Returns true if self is a small hiragana letter
    fn is_small_kana(&self) -> bool;

    fn is_particle(&self) -> bool;
}

impl JapaneseExt for char {
    #[inline]
    fn is_katakana(&self) -> bool {
        (*self) >= '\u{30A0}' && (*self) <= '\u{30FF}'
    }

    #[inline]
    fn is_hiragana(&self) -> bool {
        (*self) >= '\u{3040}' && (*self) <= '\u{309F}'
    }

    #[inline]
    fn is_kana(&self) -> bool {
        self.is_hiragana() || self.is_katakana()
    }

    #[inline]
    fn has_roman_letter(&self) -> bool {
        self.is_roman_letter()
    }

    #[inline]
    fn is_roman_letter(&self) -> bool {
        (*self) >= '\u{FF01}' && (*self) <= '\u{FF5A}'
            || ((*self) >= '\u{2000}' && (*self) <= '\u{206F}')
            || ((*self) >= '\u{20000}' && (*self) <= '\u{2A6DF}')
            || (*self) == '\u{2010}'
            || (*self) == '\u{2212}'
    }

    #[inline]
    fn is_kanji(&self) -> bool {
        ((*self) >= '\u{3400}' && (*self) <= '\u{4DBF}')
            || ((*self) >= '\u{4E00}' && (*self) <= '\u{9FFF}')
            || ((*self) >= '\u{F900}' && (*self) <= '\u{FAFF}')
            || ((*self) >= '\u{FF10}' && (*self) <= '\u{FF19}')
            || ((*self) >= '\u{20000}' && (*self) <= '\u{2A6DF}')
            || (*self) == '\u{29E8A}'
    }

    #[inline]
    fn is_symbol(&self) -> bool {
        ((*self) >= '\u{3000}' && (*self) <= '\u{303F}')
            || ((*self) >= '\u{0370}' && (*self) <= '\u{03FF}')
            || ((*self) >= '\u{25A0}' && (*self) <= '\u{25FF}')
            || ((*self) >= '\u{FF00}' && (*self) <= '\u{FFEF}')
            || (*self) == '\u{002D}'
            || (*self) == '\u{3005}'
            || (*self) == '\u{00D7}'
    }

    #[inline]
    fn has_symbol(&self) -> bool {
        self.is_symbol()
    }

    #[inline]
    fn has_kana(&self) -> bool {
        self.is_kana()
    }

    #[inline]
    fn has_kanji(&self) -> bool {
        self.is_kanji()
    }

    #[inline]
    fn is_japanese(&self) -> bool {
        self.is_kana() || self.is_kanji() || self.is_symbol() || self.is_roman_letter()
    }

    #[inline]
    fn has_japanese(&self) -> bool {
        self.is_japanese()
    }

    #[inline]
    fn kanji_count(&self) -> usize {
        if self.is_kanji() {
            1
        } else {
            0
        }
    }

    #[inline]
    fn is_small_hiragana(&self) -> bool {
        *self == '\u{3083}' || *self == '\u{3085}' || *self == '\u{3087}'
    }

    #[inline]
    fn is_small_katakana(&self) -> bool {
        *self == '\u{30E3}' || *self == '\u{30E5}' || *self == '\u{30E7}'
    }

    #[inline]
    fn is_small_kana(&self) -> bool {
        self.is_small_katakana() || self.is_small_hiragana()
    }

    #[inline]
    fn is_particle(&self) -> bool {
        matches!(
            self,
            'を' | 'の' | 'に' | 'と' | 'が' | 'か' | 'は' | 'も' | 'で' | 'へ' | 'や'
        )
    }
}

impl JapaneseExt for str {
    #[inline]
    fn is_hiragana(&self) -> bool {
        !self.chars().into_iter().any(|s| !s.is_hiragana())
    }

    #[inline]
    fn is_katakana(&self) -> bool {
        !self.chars().into_iter().any(|s| !s.is_katakana())
    }

    #[inline]
    fn is_roman_letter(&self) -> bool {
        !self.chars().into_iter().any(|s| !s.is_roman_letter())
    }

    #[inline]
    fn has_roman_letter(&self) -> bool {
        self.chars().into_iter().any(|s| s.is_roman_letter())
    }

    #[inline]
    fn has_kana(&self) -> bool {
        self.chars().into_iter().any(|s| s.is_kana())
    }

    #[inline]
    fn has_symbol(&self) -> bool {
        self.chars().into_iter().any(|s| s.is_symbol())
    }

    #[inline]
    fn is_kana(&self) -> bool {
        !self.chars().into_iter().any(|s| !s.is_kana())
    }

    #[inline]
    fn is_kanji(&self) -> bool {
        !self.chars().into_iter().any(|s| !s.is_kanji())
    }

    #[inline]
    fn has_kanji(&self) -> bool {
        self.chars().into_iter().any(|s| s.is_kanji())
    }

    #[inline]
    fn is_japanese(&self) -> bool {
        let mut buf = [0; 16];
        !self.chars().into_iter().any(|c| {
            let s = c.encode_utf8(&mut buf);
            !s.is_kana() && !s.is_kanji() && !s.is_symbol() && !s.is_roman_letter()
        })
    }

    #[inline]
    fn has_japanese(&self) -> bool {
        let mut buf = [0; 16];
        self.chars().into_iter().any(|c| {
            let s = c.encode_utf8(&mut buf);
            s.is_kana() || s.is_kanji() || s.is_symbol() || s.is_roman_letter()
        })
    }

    #[inline]
    fn kanji_count(&self) -> usize {
        self.chars().into_iter().filter(|i| i.is_kanji()).count()
    }

    #[inline]
    fn is_symbol(&self) -> bool {
        !self.chars().into_iter().any(|s| !s.is_symbol())
    }

    #[inline]

    fn is_small_katakana(&self) -> bool {
        !self.chars().into_iter().any(|s| !s.is_small_katakana())
    }

    #[inline]
    fn is_small_hiragana(&self) -> bool {
        !self.chars().into_iter().any(|s| !s.is_small_hiragana())
    }

    #[inline]
    fn is_small_kana(&self) -> bool {
        self.is_small_katakana() || self.is_small_hiragana()
    }

    #[inline]
    fn is_particle(&self) -> bool {
        !self.chars().into_iter().any(|s| !s.is_particle())
    }
}
