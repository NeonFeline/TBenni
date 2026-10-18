use crate::exceptions::{
    is_antepenultimate_stress_lowercase, is_latin_nia_glide, lookup_loanword_override,
};
use crate::phoneme::Phoneme;
use crate::strutil::{contains, has_prefix, has_suffix};
use crate::types::{Config, Dialect, StressMode};

use std::borrow::Cow;

/// Convert a single orthographic word into a vector of phonemes.
pub fn phonemize_word(word: &str, config: &Config) -> Vec<Phoneme> {
    // Fast path: lowercase ASCII letters (plus ' and -) need no cleaning
    let plain_ascii = word.bytes().all(|b| b.is_ascii_lowercase() || b == b'-' || b == b'\'');
    let cleaned: Cow<str> = if plain_ascii {
        Cow::Borrowed(word)
    } else if word.chars().any(|c| c.is_uppercase()) {
        Cow::Owned(
            word.to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic() || *c == '-' || *c == '\'')
                .collect(),
        )
    } else if word.chars().any(|c| !c.is_alphabetic() && c != '-' && c != '\'') {
        Cow::Owned(
            word.chars()
                .filter(|c| c.is_alphabetic() || *c == '-' || *c == '\'')
                .collect(),
        )
    } else {
        Cow::Borrowed(word)
    };

    if cleaned.is_empty() {
        return Vec::new();
    }

    // 1. Check curated loanword exception lexicon first
    if let Some(exception) = lookup_loanword_override(&cleaned) {
        let mut result = exception.to_vec();
        if config.include_stress {
            apply_stress(&mut result, &cleaned, config);
        }
        return result;
    }

    // 1b. Vowelless abbreviations are read as letter names (PKB -> [pɛkaˈbɛ])
    if let Some(mut spelled) = spell_abbreviation(&cleaned) {
        apply_regressive_assimilation(&mut spelled);
        apply_final_devoicing(&mut spelled);
        if config.include_stress {
            stress_last_syllable(&mut spelled);
        }
        return spelled;
    }

    // 2. Pre-process word: Morpheme boundary detachment for prefixes and special stems
    let prepared = preprocess_boundaries(&cleaned);

    // 3. Pass 1: Grapheme-to-base-phoneme conversion
    let mut phonemes = convert_graphemes(&prepared, config);

    // 4. Pass 2: Contextual nasal vowel decomposition
    decompose_nasal_vowels(&mut phonemes, config);

    // 5. Pass 3: Progressive devoicing of /v/ and /ʐ/ after voiceless obstruents
    apply_progressive_devoicing(&mut phonemes);

    // 6. Pass 4: Regressive voicing assimilation in consonant clusters
    apply_regressive_assimilation(&mut phonemes);

    // 7. Pass 5: Word-final devoicing (apocope)
    apply_final_devoicing(&mut phonemes);

    // 8. Pass 6: Silesian specific dialectal phonetic transformations
    if config.dialect == Dialect::Slask {
        apply_silesian_transformations(&mut phonemes);
    }

    // 9. Pass 7: Syllabification and stress placement
    if config.include_stress {
        apply_stress(&mut phonemes, &cleaned, config);
    }

    phonemes
}

/// Whole-word respellings into regular Polish orthography, sorted for binary search.
/// '#' blocks digraph merging.
static RESPELL_WORDS: &[(&str, &str)] = &[
    ("cennik", "ceńnik"),
    ("codziennie", "codzieńnie"),
    ("dick", "dik"),
    ("dziennie", "dzieńnie"),
    ("inni", "ińni"),
    ("may", "mej"),
    ("mógł", "mókł"),
    ("szedł", "szetł"),
    ("whisky", "łiski"),
];

/// Lexical stems respelled wherever they start the word or follow a verbal prefix
/// (skonsultować, zdiagnozować, niezidentyfikowany). Nasal stems were mined from the
/// development lexicons (Wikipron + MFA words outside dev/test) and kept only where the
/// nasal vowel is used consistently: konsekwencja [kɔw̃sɛkfɛnt͡sja], but konstrukcja [kɔnstr-].
static RESPELL_STEMS: &[(&str, &str)] = &[
    ("alfons", "alfąs"),
    ("benzyn", "bęzyn"),
    ("cotangens", "cotangęs"),
    ("cross", "kros"),
    ("demonstran", "demąstran"),
    ("diagno", "di#agno"),
    ("einstein", "ajnsztajn"),
    ("energizer", "enerdżajzer"),
    ("enzym", "ęzym"),
    ("intensyw", "intęsyw"),
    ("klient", "klijent"),
    ("kokain", "kokajin"),
    ("konfer", "kąfer"),
    ("konfigur", "kąfigur"),
    ("konfisk", "kąfisk"),
    ("konflikt", "kąflikt"),
    ("konform", "kąform"),
    ("konsek", "kąsek"),
    ("konsens", "kąsęs"),
    ("konstan", "kąstan"),
    ("konstel", "kąstel"),
    ("konsul", "kąsul"),
    ("konwal", "kąwal"),
    ("konwenc", "kąwenc"),
    ("kredens", "kredęs"),
    ("mail", "mejl"),
    ("manager", "menedżer"),
    ("nonsens", "nąsęs"),
    ("ofensyw", "ofęsyw"),
    ("online", "onlajn"),
    ("pizz", "pic"),
    ("popcorn", "popkorn"),
    ("rozdziel", "roździel"),
    ("siemens", "siemęs"),
    ("spaghett", "spaget"),
    ("tangens", "tangęs"),
    ("tensor", "tęsor"),
    ("weekend", "wikend"),
    ("whisk", "łisk"),
    ("zidentyfik", "z#identyfik"),
    ("zignor", "z#ignor"),
    ("zilustr", "z#ilustr"),
    ("zimprowiz", "z#improwiz"),
    ("zintegr", "z#integr"),
    ("zinterpret", "z#interpret"),
    ("zirytow", "z#irytow"),
    ("zizolow", "z#izolow"),
    ("zombie", "zombi"),
];

/// Prefixes after which a respelled stem may start.
const STEM_PREFIXES: &[&str] = &[
    "nie", "s", "z", "ze", "po", "prze", "przy", "od", "roz", "wy", "za", "na", "do", "u", "re", "anty",
];

fn match_stem(word: &str, start: usize) -> Option<(usize, &'static str, &'static str)> {
    let rest = &word[start..];
    let first = *rest.as_bytes().first()?;
    // RESPELL_STEMS is sorted, so stems sharing the first byte form one contiguous run
    let lo = RESPELL_STEMS.partition_point(|(stem, _)| stem.as_bytes()[0] < first);
    RESPELL_STEMS[lo..]
        .iter()
        .take_while(|(stem, _)| stem.as_bytes()[0] == first)
        .find(|(stem, _)| has_prefix(rest, stem))
        .map(|(stem, rep)| (start, *stem, *rep))
}

fn apply_respelling(word: &str) -> Option<String> {
    if let Ok(idx) = RESPELL_WORDS.binary_search_by(|(w, _)| crate::strutil::cmp(w, word)) {
        return Some(RESPELL_WORDS[idx].1.to_string());
    }
    let mut found = match_stem(word, 0);
    if found.is_none() && matches!(word.as_bytes().first(), Some(b'n' | b's' | b'z' | b'p' | b'o' | b'r' | b'w' | b'd' | b'u' | b'a')) {
        for p in STEM_PREFIXES {
            if has_prefix(word, p) {
                found = match_stem(word, p.len());
                if found.is_none() && *p == "nie" {
                    found = STEM_PREFIXES
                        .iter()
                        .filter(|q| **q != "nie" && has_prefix(&word[3..], q))
                        .find_map(|q| match_stem(word, 3 + q.len()));
                }
                if found.is_some() {
                    break;
                }
            }
        }
    }
    found.map(|(start, stem, rep)| {
        let mut out = String::with_capacity(word.len() + 4);
        out.push_str(&word[..start]);
        out.push_str(rep);
        out.push_str(&word[start + stem.len()..]);
        out
    })
}

/// Pre-process morpheme boundaries to prevent illegal affricatization or digraph merging.
/// Replaces boundaries with a delimiter '#' that instructs the converter not to merge.
fn preprocess_boundaries(word: &str) -> Cow<'_, str> {
    if let Some(respelled) = apply_respelling(word) {
        return Cow::Owned(preprocess_morphemes(&respelled).into_owned());
    }
    preprocess_morphemes(word)
}

fn preprocess_morphemes(word: &str) -> Cow<'_, str> {
    // Check for marznąć family: 'rz' is [r-z], NOT [ʐ]!
    // ("zamarz-"/"przemarz-" alone would also catch zamarzyć "to dream" [zamaʐɨt͡ɕ].)
    let marz_stems = [
        "marzn", "marzł", "marzli", "marzły", "marzła", "marznię", "marznie",
        "zmarzlak", "zmarzli", "zmarzł", "odmarzli",
        "odmarza", "wymarza", "przemarza", "zamarza", "marzać", "marzaj", "marzał",
        "mierzi", "obmierz", "murzasi",
    ];
    if contains(word, "rz") {
        for stem in marz_stems {
            if contains(word, stem) {
                let target = stem.replace("rz", "r#z");
                return Cow::Owned(word.replace(stem, &target));
            }
        }
        // The name Tarzan [tarzan], but not tarzać się / powtarzany [taʐa-]
        if has_prefix(word, "tarzan") && !has_prefix(word, "tarzani") && !has_prefix(word, "tarzany") {
            return Cow::Owned(word.replacen("tarzan", "tar#zan", 1));
        }
    }

    // Numeral compounds in -dziesiąt: elide 'ć' in base forms (pięćdziesiąt -> [pʲɛɲd͡ʑɛɕɔnt])
    let num_stems = [
        ("pięćdziesiąt", "piędziesiąt"),
        ("pięćdziesiąty", "piędziesiąty"),
        ("sześćdziesiąt", "szeździesiąt"),
        ("sześćdziesiąty", "szeździesiąty"),
        ("dziewięćdziesiąt", "dziewiędziesiąt"),
        ("dziewięćdziesiąty", "dziewiędziesiąty"),
    ];
    if has_suffix(word, "dziesiąt") || has_suffix(word, "dziesiąty") {
        for (orig, repl) in num_stems {
            if word == orig {
                return Cow::Borrowed(repl);
            }
        }
    }

    // Check for prefixes ending with 'd': od-, pod-, nad-, przed-
    // when followed by z, ż, ź, s, sz, cz
    let d_prefixes = ["od", "pod", "nad", "przed"];
    for prefix in d_prefixes {
        if has_prefix(word, prefix) {
            let rest = &word[prefix.len()..];
            // Exclude 'zi' because in Polish dzi is the single affricate [d͡ʑ]
            // e.g. podzielić, podział, nadzieja, odzież, przedział
            if rest.starts_with("zi") {
                continue;
            }
            if rest.starts_with('z')
                || rest.starts_with('ż')
                || rest.starts_with('ź')
                || rest.starts_with('s')
                || rest.starts_with("cz")
                || rest.starts_with("sz")
            {
                // Detach boundary: e.g. "od-znaczyć" -> "od#znaczyć"
                return Cow::Owned(format!("{}#{}", prefix, rest));
            }
        }
    }

    Cow::Borrowed(word)
}

/// Pass 1: Grapheme to base phoneme mapping using contextual lookaheads.
fn convert_graphemes(input: &str, _config: &Config) -> Vec<Phoneme> {
    let mut stack_buf = ['\0'; 64];
    let heap_buf;
    let chars: &[char] = if input.len() <= 64 {
        let mut count = 0;
        for c in input.chars() {
            if count < 64 {
                stack_buf[count] = c;
                count += 1;
            }
        }
        &stack_buf[..count]
    } else {
        heap_buf = input.chars().collect::<Vec<char>>();
        &heap_buf
    };
    let n = chars.len();
    let mut result = Vec::with_capacity(n + 4);
    let mut i = 0;

    let is_vowel_char = |c: char| matches!(c, 'a' | 'ą' | 'e' | 'ę' | 'i' | 'o' | 'ó' | 'u' | 'y');

    while i < n {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        let next2 = chars.get(i + 2).copied();

        // Boundary marker
        if c == '#' {
            i += 1;
            continue;
        }

        // Anglicism root: 'dubbing-' / 'zdubbing-' -> [dabʲiŋk] (coda) / [dabʲiŋɡ-] (onset)
        if c == 'd' && next == Some('u') && chars[i..].starts_with(&['d', 'u', 'b', 'b', 'i', 'n', 'g']) {
            result.push(Phoneme::D);
            result.push(Phoneme::A);
            result.push(Phoneme::BPal);
            result.push(Phoneme::I);
            result.push(Phoneme::N);
            i += 6; // leave 'g' for regular onset/coda handling
            continue;
        }

        // Sanskrit loanwords starting with 'bh-' -> [b] (e.g. Bhutan, bhutański, bhakti)
        if c == 'b' && next == Some('h') {
            result.push(Phoneme::B);
            i += 2;
            continue;
        }

        // Tri-graphs / multi-letter sequences
        if c == 'd' && next == Some('z') && next2 == Some('i') {
            let next3 = chars.get(i + 3).copied();
            let is_before_vowel = next3.map_or(false, is_vowel_char);
            result.push(Phoneme::Dzi);
            if is_before_vowel {
                i += 3; // 'dzi' before vowel -> [d͡ʑ], 'i' is palatalizer only
            } else {
                result.push(Phoneme::I);
                i += 3; // 'dzi' before consonant/end -> [d͡ʑ, i]
            }
            continue;
        }

        // Foreign digraphs: word-initial 'aal-' / 'aar-' -> [ɔ] (e.g. Aalborg, Aarhus)
        if i == 0 && c == 'a' && next == Some('a') && matches!(next2, Some('l') | Some('r')) {
            result.push(Phoneme::O);
            i += 2;
            continue;
        }

        // Compound hiatus 'oo' -> [ɔʔɔ] (e.g. bystrooki, dookoła, samoobrona, wodoodporny)
        // vs English borrowings with 'oo' -> [u] (e.g. audiobook, boom, boombox, cool, snooker)
        if c == 'o' && next == Some('o') {
            let is_english_oo = (i >= 1 && matches!(chars[i - 1], 'b' | 'l') && chars[i..].starts_with(&['o', 'o', 'k']))
                || chars[i..].starts_with(&['o', 'o', 'l'])
                || chars[i..].starts_with(&['o', 'o', 'm'])
                || (i >= 1 && chars[i - 1] == 'f' && chars[i..].starts_with(&['o', 'o', 'd']))
                || (i >= 2 && &chars[i - 2..i + 2] == &['s', 'n', 'o', 'o'])
                || (i >= 1 && matches!(chars[i - 1], 'v' | 'w' | 'g' | 'h') && chars[i..].starts_with(&['o', 'o', 'd']));
            if is_english_oo {
                result.push(Phoneme::U);
                i += 2;
                continue;
            } else {
                result.push(Phoneme::O);
                result.push(Phoneme::GlottalStop);
                result.push(Phoneme::O);
                i += 2;
                continue;
            }
        }

        // Foreign 'th' -> [t] (Beethoven, Lesotho, thriller, Rutherford)
        if c == 't' && next == Some('h') {
            result.push(Phoneme::T);
            i += 2;
            continue;
        }

        // Digraphs
        if c == 'c' && next == Some('z') {
            result.push(Phoneme::Cz);
            i += 2;
            continue;
        }
        if c == 's' && next == Some('z') {
            result.push(Phoneme::Sz);
            i += 2;
            continue;
        }
        if c == 's' && next == Some('h') {
            let is_prefix = (i >= 2 && chars[i - 2] == 'd' && chars[i - 1] == 'y')
                || (i >= 2 && chars[i - 2] == 'e' && chars[i - 1] == 'k');
            if is_prefix {
                result.push(Phoneme::S);
                result.push(Phoneme::Ch);
            } else {
                result.push(Phoneme::Sz);
            }
            i += 2;
            continue;
        }
        if c == 'r' && next == Some('z') {
            result.push(Phoneme::Rz);
            i += 2;
            continue;
        }
        if c == 'c' && next == Some('h') {
            if next2 == Some('i') {
                let next3 = chars.get(i + 3).copied();
                let is_before_vowel = next3.map_or(false, is_vowel_char);
                result.push(Phoneme::ChPal);
                if is_before_vowel {
                    result.push(Phoneme::J);
                    i += 3;
                } else {
                    result.push(Phoneme::I);
                    i += 3;
                }
                continue;
            }
            result.push(Phoneme::Ch);
            i += 2;
            continue;
        }
        if c == 'd' && next == Some('ż') {
            result.push(Phoneme::DzRetro);
            i += 2;
            continue;
        }
        if c == 'd' && next == Some('ź') {
            result.push(Phoneme::Dzi);
            i += 2;
            continue;
        }
        if c == 'd' && next == Some('z') {
            result.push(Phoneme::Dz);
            i += 2;
            continue;
        }

        // 'ci', 'si', 'zi', 'ni'
        if c == 'c' && next == Some('i') {
            let is_before_vowel = next2.map_or(false, is_vowel_char);
            result.push(Phoneme::Ci);
            if is_before_vowel {
                i += 2;
            } else {
                result.push(Phoneme::I);
                i += 2;
            }
            continue;
        }
        if c == 's' && next == Some('i') {
            let is_before_vowel = next2.map_or(false, is_vowel_char);
            result.push(Phoneme::Si);
            if is_before_vowel {
                i += 2;
            } else {
                // Fixes eSpeak NG Bug #2295! 'si' before consonant is [ɕ, i]
                result.push(Phoneme::I);
                i += 2;
            }
            continue;
        }
        if c == 'z' && next == Some('i') {
            let is_before_vowel = next2.map_or(false, is_vowel_char);
            // Prefixes bez- and dez- before 'i' preserve hard [z] in cultured orthoepy (e.g. bezimienny, dezinformacja)
            let is_hard_z_prefix = i == 2
                && ((chars[0] == 'b' && chars[1] == 'e')
                    || (chars[0] == 'd' && chars[1] == 'e'));
            if is_hard_z_prefix {
                result.push(Phoneme::Z);
                result.push(Phoneme::I);
                i += 2;
                continue;
            }
            result.push(Phoneme::Zi);
            if is_before_vowel {
                i += 2;
            } else {
                // Fixes eSpeak NG Bug #2295! 'zi' before consonant (e.g. obraził, zima) is [ʑ, i]
                result.push(Phoneme::I);
                i += 2;
            }
            continue;
        }
        if c == 'n' && next == Some('i') {
            let is_before_vowel = next2.map_or(false, is_vowel_char);
            result.push(Phoneme::Ni);
            if is_before_vowel {
                // In Polish native words, 'ni' before vowel is pure [ɲ] (e.g. ania, bania, dynia, grania, pustynia).
                // Glide [ɲja] is restricted strictly to Latin/Greek borrowings and geographic proper names (e.g. albania, hiszpania, harmonia, linia).
                // Genitive/dative '-nii' always has the glide (litanii -> [litaɲji]).
                if next2 == Some('i') || is_latin_nia_glide(input) {
                    result.push(Phoneme::J);
                }
                i += 2;
            } else {
                result.push(Phoneme::I);
                i += 2;
            }
            continue;
        }

        // Palatalized labials and velars: pi, bi, fi, wi, mi, ki, gi, li, chi
        if next == Some('i') {
            let is_before_vowel = next2.map_or(false, is_vowel_char);
            let palatal_consonant = match c {
                'p' => Some(Phoneme::PPal),
                'b' => Some(Phoneme::BPal),
                'f' => Some(Phoneme::FPal),
                'w' => Some(Phoneme::VPal),
                'm' => Some(Phoneme::MPal),
                'k' => Some(Phoneme::KPal),
                'g' => Some(Phoneme::GPal),
                'l' => Some(Phoneme::LPal),
                'h' => Some(Phoneme::ChPal),
                _ => None,
            };

            if let Some(pal) = palatal_consonant {
                result.push(pal);
                if is_before_vowel {
                    // Distilled Rule: Velars before non-e vowels insert glide [j] (e.g. alergia -> [alɛrɡʲja], celiakia -> [t͡sɛlʲakʲja])
                    // Also foreign roots with hie- (hiena -> [xʲjɛna], hierarchia -> [xʲjɛrarxʲja])
                    let is_velar = matches!(c, 'k' | 'g' | 'h');
                    let is_non_e = next2 != Some('e') && next2 != Some('ę');
                    let is_hie = c == 'h' && (next2 == Some('e') || next2 == Some('ę'));
                    // '-ii' after any consonant keeps the glide (armii -> [armʲji], kastylii -> [kastɨlʲji])
                    let is_ii = next2 == Some('i');
                    if (is_velar && (is_non_e || is_hie)) || is_ii {
                        result.push(Phoneme::J);
                    }
                    i += 2;
                } else {
                    result.push(Phoneme::I);
                    i += 2;
                }
                continue;
            }
        }

        // Palatalized trill in borrowings with 'ri' -> [rʲi] before consonant, [rʲj] before vowel (e.g. barista, catering, arizona, adrian, historia)
        if c == 'r' && next == Some('i') {
            let is_before_vowel = next2.map_or(false, is_vowel_char);
            result.push(Phoneme::RPal);
            if is_before_vowel {
                result.push(Phoneme::J);
            } else {
                result.push(Phoneme::I);
            }
            i += 2;
            continue;
        }

        // Borrowings with coronal consonants + 'i' before vowel: 'di', 'ti' -> [d, j], [t, j] (e.g. radio, partia, abiturient)
        if (c == 'd' || c == 't') && next == Some('i') && next2.map_or(false, is_vowel_char) {
            // '-tii' has no glide (partii -> [partʲi], kwestii), unlike '-dii' (komedii -> [kɔmɛdji])
            if c == 't' && next2 == Some('i') {
                result.push(Phoneme::T);
                i += 2;
                continue;
            }
            match c {
                'd' => result.push(Phoneme::D),
                't' => result.push(Phoneme::T),
                _ => {}
            }
            result.push(Phoneme::J);
            i += 2;
            continue;
        }

        // Single letters
        match c {
            'a' => result.push(Phoneme::A),
            'ą' => result.push(Phoneme::ONasal), // decomposed in Pass 2
            'b' => result.push(Phoneme::B),
            'c' => result.push(Phoneme::C),
            'ć' => result.push(Phoneme::Ci),
            'd' => result.push(Phoneme::D),
            'e' => result.push(Phoneme::E),
            'ę' => result.push(Phoneme::ENasal), // decomposed in Pass 2
            'f' => result.push(Phoneme::F),
            'g' => result.push(Phoneme::G),
            'h' => result.push(Phoneme::Ch),
            'i' => result.push(Phoneme::I),
            'j' => result.push(Phoneme::J),
            'k' => result.push(Phoneme::K),
            'l' => result.push(Phoneme::L),
            'ł' => result.push(Phoneme::W),
            'm' => result.push(Phoneme::M),
            'n' => result.push(Phoneme::N),
            'ń' => result.push(Phoneme::Ni),
            'o' => result.push(Phoneme::O),
            'ó' => result.push(Phoneme::U),
            'p' => result.push(Phoneme::P),
            'r' => result.push(Phoneme::R),
            's' => result.push(Phoneme::S),
            'ś' => result.push(Phoneme::Si),
            't' => result.push(Phoneme::T),
            'u' => {
                // Distilled rule: Diphthongization in 'au', 'eu' -> [aw], [ɛw] (auto -> [awtɔ], Europa -> [ɛwrɔpa])
                // Exceptions: Latin endings in -eum (muzeum, apogeum, liceum), -eusz (ateusz, mateusz), or aleut-
                let is_prev_vowel = matches!(result.last(), Some(Phoneme::A) | Some(Phoneme::E) | Some(Phoneme::ASlant) | Some(Phoneme::ESlant));
                let is_eum_hiatus = matches!(result.last(), Some(Phoneme::E))
                    && ((next == Some('m') && i + 2 == n)
                        || (next == Some('s') && next2 == Some('z'))
                        || (next == Some('t') && i + 2 == n));
                // Morphological prefix hiatus: na-u (nauka), za-u (zaufać), nie-u (nieudany), po-u (pouczać)
                let is_prefix_hiatus = (i == 2 && chars[i - 1] == 'a' && matches!(chars[0], 'n' | 'z'))
                    || (i == 3 && chars[i - 1] == 'e' && chars[i - 2] == 'i' && chars[0] == 'n')
                    || (i == 2 && chars[i - 1] == 'o' && chars[0] == 'p')
                    || (i > 0 && chars[i - 1] == '#');
                if is_prev_vowel && !is_eum_hiatus && !is_prefix_hiatus {
                    result.push(Phoneme::W);
                } else {
                    result.push(Phoneme::U);
                }
            }
            'w' => result.push(Phoneme::V),
            'y' => result.push(Phoneme::Y),
            'z' => result.push(Phoneme::Z),
            'ź' => result.push(Phoneme::Zi),
            'ż' => result.push(Phoneme::ZRetro),
            // Foreign / non-standard Latin letters
            'v' => result.push(Phoneme::V),
            'x' => {
                result.push(Phoneme::K);
                result.push(Phoneme::S);
            }
            'q' => {
                result.push(Phoneme::K);
                if next == Some('u') {
                    if matches!(next2, Some('a') | Some('o') | Some('ó')) {
                        result.push(Phoneme::F);
                    } else {
                        result.push(Phoneme::W);
                    }
                    i += 2;
                    continue;
                }
            }
            // Silesian specific orthography characters
            'ō' => result.push(Phoneme::OSlant),
            'é' => result.push(Phoneme::ESlant),
            'á' => result.push(Phoneme::ASlant),
            'ů' => result.push(Phoneme::U),
            _ => {}
        }

        i += 1;
    }

    result
}

/// Pass 2: Contextual nasal vowel decomposition (ząb -> zomp, ręka -> renka/reŋka, wziął -> wzioł).
fn decompose_nasal_vowels(phonemes: &mut Vec<Phoneme>, config: &Config) {
    let mut i = 0;
    while i < phonemes.len() {
        let p = phonemes[i];
        if p == Phoneme::ONasal || p == Phoneme::ENasal {
            let is_o = p == Phoneme::ONasal;
            let base_vowel = if is_o { Phoneme::O } else { Phoneme::E };
            let next_p = phonemes.get(i + 1).copied();

            match next_p {
                // 1. Bilabials (p, b, pʲ, bʲ) -> homorganic [m]
                Some(Phoneme::P | Phoneme::B | Phoneme::PPal | Phoneme::BPal) => {
                    phonemes[i] = base_vowel;
                    phonemes.insert(i + 1, Phoneme::M);
                    i += 2;
                    continue;
                }
                // 2. Dentals (t, d, c, dz) -> homorganic [n]
                Some(Phoneme::T | Phoneme::D | Phoneme::C | Phoneme::Dz) => {
                    phonemes[i] = base_vowel;
                    phonemes.insert(i + 1, Phoneme::N);
                    i += 2;
                    continue;
                }
                // 3. Retroflex / Postalveolar affricates (cz, dż) -> homorganic [n̠]
                Some(Phoneme::Cz | Phoneme::DzRetro) => {
                    phonemes[i] = base_vowel;
                    phonemes.insert(i + 1, Phoneme::NRetro);
                    i += 2;
                    continue;
                }
                // 4. Alveolo-palatals (ć, dź) -> homorganic [ɲ]
                Some(Phoneme::Ci | Phoneme::Dzi) => {
                    phonemes[i] = base_vowel;
                    phonemes.insert(i + 1, Phoneme::Ni);
                    i += 2;
                    continue;
                }
                // 5. Velars (k, g, kʲ, gʲ) -> homorganic [ŋ]
                Some(Phoneme::K | Phoneme::G | Phoneme::KPal | Phoneme::GPal) => {
                    phonemes[i] = base_vowel;
                    phonemes.insert(i + 1, Phoneme::Ng);
                    i += 2;
                    continue;
                }
                // 6. Liquids (l, ł) -> complete denasalization
                Some(Phoneme::L | Phoneme::LPal | Phoneme::W) => {
                    phonemes[i] = base_vowel;
                    i += 1;
                    continue;
                }
                // 7. Fricatives (s, z, sz, ż, ś, ź, f, w, ch) -> diphthongal nasal glide
                Some(
                    Phoneme::S
                    | Phoneme::Z
                    | Phoneme::Sz
                    | Phoneme::ZRetro
                    | Phoneme::Si
                    | Phoneme::Zi
                    | Phoneme::F
                    | Phoneme::V
                    | Phoneme::Ch
                    | Phoneme::ChPal,
                ) => {
                    // Retain diphthongal glide [ɔw̃] / [ɛw̃]
                    i += 1;
                    continue;
                }
                // 8. Word-final position (None)
                None => {
                    if is_o {
                        // Word-final 'ą' preserves diphthong [ɔw̃] in cultured Polish
                        i += 1;
                        continue;
                    } else {
                        // Word-final 'ę' denasalizes to [ɛ] in cultured Polish
                        if config.denasalize_final_e {
                            phonemes[i] = Phoneme::E;
                        }
                        i += 1;
                        continue;
                    }
                }
                _ => {
                    i += 1;
                    continue;
                }
            }
        }
        i += 1;
    }
}

/// Interjections without vowel letters that are pronounced as written, not spelled out.
const VOWELLESS_INTERJECTIONS: &[&str] = &[
    "hm", "hmm", "mhm", "brr", "pst", "psst", "sza", "szsz", "tss", "cyt", "kch", "pff", "pfff", "grr",
];

/// Spells a word with no vowel letters as Polish letter names (NKWD -> [ɛnkaɛftɛ]).
fn spell_abbreviation(word: &str) -> Option<Vec<Phoneme>> {
    use Phoneme::*;
    let n_chars = word.chars().count();
    if n_chars < 2
        || n_chars > 8
        || word.chars().any(|c| matches!(c, 'a' | 'ą' | 'e' | 'ę' | 'i' | 'o' | 'ó' | 'u' | 'y' | '-' | '\''))
        || VOWELLESS_INTERJECTIONS.contains(&word)
    {
        return None;
    }
    let mut out = Vec::with_capacity(n_chars * 3);
    for c in word.chars() {
        let name: &[Phoneme] = match c {
            'b' => &[B, E],
            'c' => &[C, E],
            'ć' => &[Ci, E],
            'd' => &[D, E],
            'f' => &[E, F],
            'g' => &[GPal, E],
            'h' => &[Ch, A],
            'j' => &[J, O, T],
            'k' => &[K, A],
            'l' => &[E, L],
            'ł' => &[E, W],
            'm' => &[E, M],
            'n' => &[E, N],
            'ń' => &[E, Ni],
            'p' => &[P, E],
            'q' => &[K, U],
            'r' => &[E, R],
            's' => &[E, S],
            'ś' => &[E, Si],
            't' => &[T, E],
            'v' => &[F, A, W],
            'w' => &[V, U],
            'x' => &[I, K, S],
            'z' => &[Z, E, T],
            'ź' => &[Zi, E, T],
            'ż' => &[ZRetro, E, T],
            _ => return None,
        };
        out.extend_from_slice(name);
    }
    Some(out)
}

/// Abbreviations are stressed on the final letter name.
fn stress_last_syllable(phonemes: &mut Vec<Phoneme>) {
    if let Some(v) = phonemes.iter().rposition(|p| p.is_vowel()) {
        let mut pos = v;
        while pos > 0 && !phonemes[pos - 1].is_vowel() {
            pos -= 1;
        }
        phonemes.insert(pos, Phoneme::PrimaryStress);
    }
}

/// Pass 3: Progressive devoicing of /v/ and /ʐ/ after voiceless consonants (twój -> [tfuj], krzak -> [kʂak]).
fn apply_progressive_devoicing(phonemes: &mut [Phoneme]) {
    for i in 1..phonemes.len() {
        let prev = phonemes[i - 1];
        let curr = phonemes[i];

        if prev.is_obstruent() && !prev.is_voiced() {
            if curr == Phoneme::V {
                phonemes[i] = Phoneme::F;
            } else if curr == Phoneme::VPal {
                phonemes[i] = Phoneme::FPal;
            } else if curr == Phoneme::Rz {
                phonemes[i] = Phoneme::Sz;
            }
        }
    }
}

/// Pass 4: Regressive voicing assimilation in consonant clusters (prośba -> [prɔʑba], łódka -> [wutka]).
fn apply_regressive_assimilation(phonemes: &mut [Phoneme]) {
    if phonemes.len() < 2 {
        return;
    }

    // Traverse from right to left
    for i in (0..phonemes.len() - 1).rev() {
        let curr = phonemes[i];
        let next = phonemes[i + 1];

        // Block affricate gemination [t͡ɕd͡ʑ] -> *[d͡ʑd͡ʑ]* in numeral compounds (pięćdziesiąta, sześćdziesiąta)
        if curr == Phoneme::Ci && next == Phoneme::Dzi {
            continue;
        }

        if curr.is_obstruent() && next.is_obstruent() {
            // If next is voiced (and not /v/ which doesn't trigger regressive voicing):
            if next.is_voiced() && next != Phoneme::V && next != Phoneme::VPal {
                if !curr.is_voiced() {
                    phonemes[i] = curr.to_voiced();
                }
            } else if !next.is_voiced() {
                // If next is voiceless, curr must devoice
                if curr.is_voiced() {
                    phonemes[i] = curr.to_devoiced();
                }
            }
        }
    }
}

/// Pass 5: Word-final devoicing (apocope) of obstruents before pause (chleb -> [xlɛp], mózg -> [musk]).
fn apply_final_devoicing(phonemes: &mut [Phoneme]) {
    let mut i = phonemes.len();
    while i > 0 {
        let p = phonemes[i - 1];
        if p.is_obstruent() {
            if p.is_voiced() {
                phonemes[i - 1] = p.to_devoiced();
            }
            i -= 1;
        } else {
            break;
        }
    }
}

/// Pass 6: Silesian specific dialectal transformations.
fn apply_silesian_transformations(phonemes: &mut Vec<Phoneme>) {
    // 1. Initial /o-/ labialization: insert [w] before initial [ɔ]
    if let Some(&first) = phonemes.first() {
        if first == Phoneme::O || first == Phoneme::OSlant {
            phonemes.insert(0, Phoneme::W);
        }
    }

    // 2. Slanted vowel mappings
    for p in phonemes.iter_mut() {
        if *p == Phoneme::ESlant {
            *p = Phoneme::Y; // e pochylone -> [ɨ]
        }
    }
}

/// Pass 7: Syllabification and stress placement (default: penultimate syllable).
fn apply_stress(phonemes: &mut Vec<Phoneme>, word: &str, config: &Config) {
    // last[k] = index of the (k+1)-th vowel from the end; stress never goes further back
    let mut last = [0usize; 4];
    let mut num_vowels = 0;
    for (i, p) in phonemes.iter().enumerate().rev() {
        if p.is_vowel() {
            if num_vowels < 4 {
                last[num_vowels] = i;
            }
            num_vowels += 1;
        }
    }
    if num_vowels == 0 {
        return;
    }

    let target_vowel_idx = if num_vowels == 1 {
        // Monosyllable: stress on the only vowel
        last[0]
    } else {
        // Multi-syllable word
        if config.stress_mode == StressMode::Prescriptive {
            // Check for antepenultimate (3rd from end) or pre-antepenultimate (4th from end)
            let last_byte = word.as_bytes().last().copied();
            let clitic_tail = matches!(last_byte, Some(b'y' | b'e'));
            if num_vowels >= 4 && clitic_tail && (has_suffix(word, "byśmy") || has_suffix(word, "byście")) {
                last[3]
            } else if num_vowels >= 3
                && ((clitic_tail && (has_suffix(word, "liśmy")
                    || has_suffix(word, "liście")
                    || has_suffix(word, "łyśmy")
                    || has_suffix(word, "łyście")))
                    || is_antepenultimate_stress_lowercase(word))
            {
                last[2]
            } else {
                // Regular penultimate stress (2nd from end)
                last[1]
            }
        } else {
            // Colloquial regularized penultimate stress
            last[1]
        }
    };

    // Find the syllable onset (start of consonants preceding the target vowel)
    let mut insert_pos = target_vowel_idx;
    while insert_pos > 0 && !phonemes[insert_pos - 1].is_vowel() {
        insert_pos -= 1;
    }

    phonemes.insert(insert_pos, Phoneme::PrimaryStress);
}
