//! Shared driver for the distilled per-language phonemizers.
//!
//! A word is processed in three layers:
//! 1. **Exception slice**: sorted `(word, phones)` table searched with binary search.
//! 2. **Base mapper**: hand-written rules (`langs::base::<lang>::step`) that split the word into
//!    grapheme segments and emit phones for each.
//! 3. **CART corrections**: decision trees distilled from the neural teacher. For each grapheme
//!    segment, the tree looks at the surrounding letters and may replace the base phones.

/// Padding symbol for context positions before the start of the word.
pub const BOW: char = '^';
/// Padding symbol for context positions past the end of the word.
pub const EOW: char = '$';

/// Which layers of the engine to apply (used for the ablation study).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// Hand-written base rules only.
    Base,
    /// Base rules + distilled CART corrections.
    Cart,
    /// Base rules + CART corrections + exception slice (the default engine).
    Full,
}

impl Stage {
    pub fn parse(s: &str) -> Option<Stage> {
        match s.to_lowercase().as_str() {
            "base" => Some(Stage::Base),
            "cart" => Some(Stage::Cart),
            "full" => Some(Stage::Full),
            _ => None,
        }
    }
}

/// Everything the driver needs to know about one language.
pub struct LangSpec {
    /// Base mapper: emits the phones for the segment starting at `i`, returns the segment end.
    pub step: fn(&[char], usize, &mut String) -> usize,
    /// CART correction for the segment `chars[start..end]`; `None` keeps the base phones.
    pub cart: fn(&[char], usize, usize) -> Option<&'static str>,
    /// Exception slice, sorted by word.
    pub lexicon: &'static [(&'static str, &'static str)],
}

/// Context letter at signed offset `idx`, padded with [`BOW`] / [`EOW`].
#[inline(always)]
pub fn at(chars: &[char], idx: isize) -> char {
    if idx < 0 {
        BOW
    } else {
        chars.get(idx as usize).copied().unwrap_or(EOW)
    }
}

/// Grapheme vowel letters across the supported Latin and Cyrillic orthographies.
#[inline(always)]
pub fn is_vowel(c: char) -> bool {
    matches!(
        c,
        'a' | 'e'
            | 'i'
            | 'o'
            | 'u'
            | 'y'
            | 'á'
            | 'é'
            | 'í'
            | 'ó'
            | 'ú'
            | 'ů'
            | 'ý'
            | 'ě'
            | 'ą'
            | 'ę'
            | 'а'
            | 'е'
            | 'ё'
            | 'и'
            | 'о'
            | 'у'
            | 'ы'
            | 'э'
            | 'ю'
            | 'я'
            | 'і'
            | 'ї'
            | 'є'
            | 'ъ'
    )
}

#[inline]
pub fn lookup(spec: &LangSpec, word: &str) -> Option<&'static str> {
    spec.lexicon
        .binary_search_by_key(&word, |(w, _)| w)
        .ok()
        .map(|idx| spec.lexicon[idx].1)
}

/// Phonemize a single word with the given engine layers.
pub fn phonemize_word(spec: &LangSpec, word: &str, stage: Stage) -> String {
    let lower = word.to_lowercase();
    if stage == Stage::Full {
        if let Some(phones) = lookup(spec, &lower) {
            return phones.to_string();
        }
    }
    let chars: Vec<char> = lower.chars().collect();
    let mut out = String::with_capacity(chars.len() * 3);
    let mut i = 0;
    while i < chars.len() {
        let mark = out.len();
        let end = (spec.step)(&chars, i, &mut out);
        if stage != Stage::Base {
            if let Some(phones) = (spec.cart)(&chars, i, end) {
                out.truncate(mark);
                out.push_str(phones);
            }
        }
        i = end;
    }
    out
}

/// Base-mapper segmentation of a word: `(grapheme span, base phones)` pairs in order.
/// Used by the distillation pipeline to align teacher output with grapheme segments.
pub fn base_segments(spec: &LangSpec, word: &str) -> Vec<(String, String)> {
    let chars: Vec<char> = word.to_lowercase().chars().collect();
    let mut segs = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let mut phones = String::new();
        let end = (spec.step)(&chars, i, &mut phones);
        segs.push((chars[i..end].iter().collect(), phones));
        i = end;
    }
    segs
}
