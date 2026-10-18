//! Byte-level string helpers for the hot path. Words are short (~10 bytes), where a plain
//! loop beats both a libc `memcmp` call and the Two-Way searcher behind `str::contains`.

use std::cmp::Ordering;

#[inline]
pub(crate) fn has_prefix(s: &str, p: &str) -> bool {
    let (s, p) = (s.as_bytes(), p.as_bytes());
    s.len() >= p.len() && s.iter().zip(p).all(|(a, b)| a == b)
}

#[inline]
pub(crate) fn has_suffix(s: &str, p: &str) -> bool {
    let (s, p) = (s.as_bytes(), p.as_bytes());
    s.len() >= p.len() && s[s.len() - p.len()..].iter().zip(p).all(|(a, b)| a == b)
}

#[inline]
pub(crate) fn contains(s: &str, needle: &str) -> bool {
    let (s, n) = (s.as_bytes(), needle.as_bytes());
    if n.is_empty() || s.len() < n.len() {
        return n.is_empty();
    }
    (0..=s.len() - n.len()).any(|i| s[i] == n[0] && s[i..i + n.len()].iter().zip(n).all(|(a, b)| a == b))
}

#[inline]
pub(crate) fn cmp(a: &str, b: &str) -> Ordering {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    for (x, y) in a.iter().zip(b) {
        if x != y {
            return x.cmp(y);
        }
    }
    a.len().cmp(&b.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn helpers_match_std() {
        let words = ["", "a", "rz", "marznąć", "dziesiąt", "pięćdziesiąt", "ząb", "zrobiliśmy"];
        let pats = ["", "rz", "dziesiąt", "z", "ąb", "liśmy", "marznąć!"];
        for w in words {
            for p in pats {
                assert_eq!(has_prefix(w, p), w.starts_with(p), "{w} {p}");
                assert_eq!(has_suffix(w, p), w.ends_with(p), "{w} {p}");
                assert_eq!(contains(w, p), w.contains(p), "{w} {p}");
                assert_eq!(cmp(w, p), w.cmp(p), "{w} {p}");
            }
        }
    }
}
