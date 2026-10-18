//! TBenni: High-Performance Polish Phonemizer
//! Named in honor of Tytus Benni (1877–1935).

pub mod exceptions;
pub mod phoneme;
pub mod rules;
pub mod sandhi;
mod strutil;
pub mod types;

pub use exceptions::{is_antepenultimate_stress_word, lookup_loanword_override};
pub use phoneme::Phoneme;
pub use rules::phonemize_word;
pub use sandhi::apply_sandhi;
pub use types::{Config, Dialect, OutputFormat, StressMode};

/// Phonemize an entire text utterance with default configuration.
pub fn phonemize(text: &str) -> String {
    phonemize_with_config(text, &Config::default())
}

/// Phonemize text with a custom configuration.
pub fn phonemize_with_config(text: &str, config: &Config) -> String {
    let trimmed = text.trim();
    // Fast path: single word without whitespace or punctuation
    if !trimmed.is_empty()
        && !trimmed.contains(' ')
        && !trimmed.contains('\t')
        && trimmed.chars().all(|c| {
            c.is_ascii_alphabetic() || c == '-' || c == '\'' || (!c.is_ascii() && c.is_alphabetic())
        })
    {
        let phonemes = phonemize_word(trimmed, config);
        let mut output = String::with_capacity(phonemes.len() * 3);
        for p in &phonemes {
            output.push_str(p.render(config.format));
        }
        return output;
    }

    let mut words_with_punct: Vec<(String, Option<char>)> = Vec::new();

    // Simple tokenizer respecting punctuation
    for token in text.split_whitespace() {
        let mut clean_word = String::new();
        let mut trailing_punct = None;

        for c in token.chars() {
            if c.is_alphabetic() || c == '-' || c == '\'' || c == 'ō' || c == 'é' || c == 'á' || c == 'ů' {
                clean_word.push(c);
            } else if matches!(c, '.' | ',' | '!' | '?' | ':' | ';') {
                trailing_punct = Some(c);
            }
        }

        if !clean_word.is_empty() {
            words_with_punct.push((clean_word, trailing_punct));
        } else if let Some(p) = trailing_punct {
            words_with_punct.push((String::new(), Some(p)));
        }
    }

    if words_with_punct.is_empty() {
        return String::new();
    }

    // Phonemize each word
    let mut words_phonemes: Vec<Vec<Phoneme>> = words_with_punct
        .iter()
        .map(|(w, _)| {
            if w.is_empty() {
                Vec::new()
            } else {
                phonemize_word(w, config)
            }
        })
        .collect();

    // Apply Sandhi across word boundaries
    if config.apply_sandhi {
        let has_trailing_punct: Vec<bool> = words_with_punct
            .iter()
            .map(|(_, punct)| punct.is_some())
            .collect();
        apply_sandhi(&mut words_phonemes, &has_trailing_punct, config);
    }

    // Assemble rendered output string
    let mut output = String::new();
    let num_words = words_phonemes.len();

    for (i, phonemes) in words_phonemes.iter().enumerate() {
        for p in phonemes {
            output.push_str(p.render(config.format));
        }

        // Add trailing punctuation if any
        if let Some((_, Some(punct))) = words_with_punct.get(i) {
            output.push(*punct);
        }

        // Add word separator
        if i + 1 < num_words {
            output.push(' ');
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_affricates_vs_clusters() {
        let config = Config {
            format: OutputFormat::IpaNarrow,
            include_stress: false,
            ..Default::default()
        };
        // czysta [t͡ʂɨsta] vs trzysta [tʂɨsta]
        assert_eq!(phonemize_with_config("czysta", &config), "t͡ʂɨsta");
        assert_eq!(phonemize_with_config("trzysta", &config), "tʂɨsta");
        // dżem [d͡ʐɛm] vs drzewo [dʐɛvɔ]
        assert_eq!(phonemize_with_config("dżem", &config), "d͡ʐɛm");
        assert_eq!(phonemize_with_config("drzewo", &config), "dʐɛvɔ");
    }

    #[test]
    fn test_morpheme_boundary_rz_vs_rz_digraph() {
        let config = Config {
            format: OutputFormat::IpaNarrow,
            include_stress: false,
            ..Default::default()
        };
        // marznąć [marznɔɲt͡ɕ] (NOT [maʐnɔɲt͡ɕ]!)
        assert_eq!(phonemize_with_config("marznąć", &config), "marznɔɲt͡ɕ");
        assert_eq!(phonemize_with_config("zmarzlak", &config), "zmarzlak");
        // standard digraph: morze [mɔʐɛ]
        assert_eq!(phonemize_with_config("morze", &config), "mɔʐɛ");
    }

    #[test]
    fn test_prefix_boundary_d_z() {
        let config = Config {
            format: OutputFormat::IpaNarrow,
            include_stress: false,
            ..Default::default()
        };
        // odznaczyć [ɔdznat͡ʂɨt͡ɕ] (NOT [ɔd͡znat͡ʂɨt͡ɕ] with affricate!)
        assert_eq!(phonemize_with_config("odznaczyć", &config), "ɔdznat͡ʂɨt͡ɕ");
        assert_eq!(phonemize_with_config("odzew", &config), "ɔdzɛf");
    }

    #[test]
    fn test_espeak_bug_2295_regression() {
        let config = Config {
            format: OutputFormat::IpaNarrow,
            include_stress: false,
            ..Default::default()
        };
        // obraził [ɔbraʑiw] (eSpeak NG outputs illegal [ɔbraziw])
        assert_eq!(phonemize_with_config("obraził", &config), "ɔbraʑiw");
        // zima [ʑima]
        assert_eq!(phonemize_with_config("zima", &config), "ʑima");
        // sito [ɕitɔ]
        assert_eq!(phonemize_with_config("sito", &config), "ɕitɔ");
    }

    #[test]
    fn test_nasal_vowel_decomposition() {
        let config = Config {
            format: OutputFormat::IpaNarrow,
            include_stress: false,
            ..Default::default()
        };
        // Before bilabial: dąb -> [dɔmp] (with final devoicing)
        assert_eq!(phonemize_with_config("dąb", &config), "dɔmp");
        // Before dental: kąt -> [kɔnt]
        assert_eq!(phonemize_with_config("kąt", &config), "kɔnt");
        // Before alveolo-palatal: pięć -> [pʲɛɲt͡ɕ]
        assert_eq!(phonemize_with_config("pięć", &config), "pʲɛɲt͡ɕ");
        // Before velar: ręka -> [rɛŋka]
        assert_eq!(phonemize_with_config("ręka", &config), "rɛŋka");
        // Before liquid: wziął -> [vʑɔw] (complete denasalization)
        assert_eq!(phonemize_with_config("wziął", &config), "vʑɔw");
        // Before fricative: wąs -> [vɔw̃s]
        assert_eq!(phonemize_with_config("wąs", &config), "vɔw̃s");
        // Word final ę: idę -> [idɛ]
        assert_eq!(phonemize_with_config("idę", &config), "idɛ");
        // Word final ą: idą -> [idɔw̃]
        assert_eq!(phonemize_with_config("idą", &config), "idɔw̃");
    }

    #[test]
    fn test_voicing_assimilations() {
        let config = Config {
            format: OutputFormat::IpaNarrow,
            include_stress: false,
            ..Default::default()
        };
        // Final devoicing: chleb -> [xlɛp]
        assert_eq!(phonemize_with_config("chleb", &config), "xlɛp");
        // Regressive voicing: prośba -> [prɔʑba]
        assert_eq!(phonemize_with_config("prośba", &config), "prɔʑba");
        // Progressive devoicing: twój -> [tfuj], krzak -> [kʂak]
        assert_eq!(phonemize_with_config("twój", &config), "tfuj");
        assert_eq!(phonemize_with_config("krzak", &config), "kʂak");
    }

    #[test]
    fn test_sandhi_warsaw_vs_krakow() {
        let config_warsaw = Config {
            dialect: Dialect::Warsaw,
            format: OutputFormat::IpaNarrow,
            include_stress: false,
            apply_sandhi: true,
            ..Default::default()
        };
        let mut config_krakow = config_warsaw.clone();
        config_krakow.dialect = Dialect::KrakowPoznan;

        // "brat ojca" -> Warsaw [brat ɔjt͡sa] vs Krakow [brad ɔjt͡sa]
        assert_eq!(phonemize_with_config("brat ojca", &config_warsaw), "brat ɔjt͡sa");
        assert_eq!(phonemize_with_config("brat ojca", &config_krakow), "brad ɔjt͡sa");

        // Vowel-final words must NEVER have internal consonants modified by sandhi!
        // "kącie, a" -> [kɔɲt͡ɕɛ, a] (NEVER *kɔɲd͡ʑɛ)
        assert_eq!(phonemize_with_config("przy kącie, a", &config_krakow), "pʂɨ kɔɲt͡ɕɛ, a");
        assert_eq!(phonemize_with_config("kącie i", &config_krakow), "kɔɲt͡ɕɛ i");

        // Punctuation blocks sandhi:
        assert_eq!(phonemize_with_config("brat, ojca", &config_krakow), "brat, ɔjt͡sa");
    }

    #[test]
    fn test_silesian_dialect() {
        let config = Config {
            dialect: Dialect::Slask,
            format: OutputFormat::IpaNarrow,
            include_stress: false,
            apply_sandhi: true,
            ..Default::default()
        };
        // Initial o- labialization: okno -> [wɔknɔ]
        assert_eq!(phonemize_with_config("okno", &config), "wɔknɔ");
        // Sandhi voicing: brat ojca -> [brad w-...]
        assert!(phonemize_with_config("brat ojca", &config).starts_with("brad"));
    }

    #[test]
    fn test_benni_as_notation() {
        let config = Config {
            format: OutputFormat::Benni,
            include_stress: false,
            ..Default::default()
        };
        // čysta, xlep, dǫp
        assert_eq!(phonemize_with_config("czysta", &config), "čysta");
        assert_eq!(phonemize_with_config("chleb", &config), "xlep");
    }

    #[test]
    fn test_prescriptive_antepenultimate_stress() {
        let config = Config {
            format: OutputFormat::IpaNarrow,
            stress_mode: StressMode::Prescriptive,
            include_stress: true,
            ..Default::default()
        };
        // fizyka -> [ˈfʲizɨka] (3rd from end)
        assert_eq!(phonemize_with_config("fizyka", &config), "ˈfʲizɨka");
        // byliśmy -> [ˈbɨlʲiɕmɨ] (3rd from end)
        assert_eq!(phonemize_with_config("byliśmy", &config), "ˈbɨlʲiɕmɨ");
        // regular word: kobieta -> [kɔˈbʲɛta] (penultimate)
        assert_eq!(phonemize_with_config("kobieta", &config), "kɔˈbʲɛta");
    }

    #[test]
    fn test_distilled_borrowing_and_glide_rules() {
        let config = Config {
            format: OutputFormat::IpaNarrow,
            include_stress: false,
            ..Default::default()
        };
        // Palatalized trill [rʲj] in borrowings
        assert_eq!(phonemize_with_config("adrian", &config), "adrʲjan");
        assert_eq!(phonemize_with_config("historia", &config), "xʲistɔrʲja");
        assert_eq!(phonemize_with_config("abiturient", &config), "abʲiturʲjɛnt");
        // Velar glides [ɡʲja], [xʲja]
        assert_eq!(phonemize_with_config("alergia", &config), "alɛrɡʲja");
        assert_eq!(phonemize_with_config("anarchia", &config), "anarxʲja");
        // Latin -nia glide
        assert_eq!(phonemize_with_config("albania", &config), "albaɲja");
        assert_eq!(phonemize_with_config("agonia", &config), "aɡɔɲja");
        // Acronyms and qu
        assert_eq!(phonemize_with_config("aquapark", &config), "akfapark");
        assert_eq!(phonemize_with_config("abc", &config), "abɛt͡sɛ");
        assert_eq!(phonemize_with_config("wc", &config), "vut͡sɛ");
    }

    #[test]
    fn test_neural_distilled_rules_batch_2() {
        let config = Config {
            format: OutputFormat::IpaNarrow,
            include_stress: false,
            ..Default::default()
        };
        // 1. Prefix hiatus vs diphthongs
        assert_eq!(phonemize_with_config("nauka", &config), "nauka");
        assert_eq!(phonemize_with_config("zaufać", &config), "zaufat͡ɕ");
        assert_eq!(phonemize_with_config("nieudany", &config), "ɲɛudanɨ");
        assert_eq!(phonemize_with_config("auto", &config), "awtɔ");

        // 2. Anglicism dubbing
        assert_eq!(phonemize_with_config("dubbing", &config), "dabʲink");
        assert_eq!(phonemize_with_config("dubbingu", &config), "dabʲinɡu");
        assert_eq!(phonemize_with_config("zdubbingować", &config), "zdabʲinɡɔvat͡ɕ");

        // 3. Prefix boundary dzi
        assert_eq!(phonemize_with_config("nadzieja", &config), "nad͡ʑɛja");
        assert_eq!(phonemize_with_config("podział", &config), "pɔd͡ʑaw");
        assert_eq!(phonemize_with_config("odzież", &config), "ɔd͡ʑɛʂ");

        // 4. Native -unia vs Latin
        assert_eq!(phonemize_with_config("babunia", &config), "babuɲa");
        assert_eq!(phonemize_with_config("unia", &config), "uɲja");

        // 5. Numeral compound simplification
        assert_eq!(phonemize_with_config("pięćdziesiąt", &config), "pʲɛɲd͡ʑɛɕɔnt");
        assert_eq!(phonemize_with_config("pięćdziesiąta", &config), "pʲɛɲt͡ɕd͡ʑɛɕɔnta");
        assert_eq!(phonemize_with_config("sześćdziesiąt", &config), "ʂɛʑd͡ʑɛɕɔnt");
        assert_eq!(phonemize_with_config("sześćdziesiąta", &config), "ʂɛɕt͡ɕd͡ʑɛɕɔnta");

        // 6. Sanskrit bh-
        assert_eq!(phonemize_with_config("bhutan", &config), "butan");

        // 7. Prefix bez- before i
        assert_eq!(phonemize_with_config("bezimienny", &config), "bɛzimʲɛnnɨ");
    }
}
