//! TBenni: High-Performance Polish Phonemizer
//! Named in honor of Tytus Benni (1877–1935).

pub mod exceptions;
pub mod langs;
pub mod phoneme;
pub mod rules;
pub mod sandhi;
mod strutil;
pub mod types;

pub use exceptions::{is_antepenultimate_stress_word, lookup_loanword_override};
pub use phoneme::Phoneme;
pub use rules::phonemize_word;
pub use sandhi::apply_sandhi;
pub use types::{Config, Dialect, OutputFormat, SilesianRegion, StressMode};

/// Phonemize text for a specific language (e.g. "pl", "cs", "hr", "bg", "uk", "ru", "de").
/// Polish (and any unknown code) uses the main Polish engine with the default configuration.
pub fn phonemize_lang(text: &str, lang: &str) -> String {
    phonemize_lang_stage(text, lang, langs::engine::Stage::Full)
}

/// Like [`phonemize_lang`], but for the distilled languages only the given engine layers are
/// applied (used for the ablation study). Other languages ignore `stage`.
pub fn phonemize_lang_stage(text: &str, lang: &str, stage: langs::engine::Stage) -> String {
    let lang: std::borrow::Cow<str> = if lang.bytes().any(|b| b.is_ascii_uppercase()) {
        std::borrow::Cow::Owned(lang.to_lowercase())
    } else {
        std::borrow::Cow::Borrowed(lang)
    };
    if let Some(spec) = langs::spec(&lang) {
        return text
            .split_whitespace()
            .map(|tok| {
                let chars: Vec<char> = tok.chars().collect();
                let lead_len = chars.iter().take_while(|&&c| !c.is_alphanumeric()).count();
                if lead_len == chars.len() {
                    return tok.to_string();
                }
                let trail_len = chars
                    .iter()
                    .rev()
                    .take_while(|&&c| !c.is_alphanumeric())
                    .count();
                let prefix: String = chars[..lead_len].iter().collect();
                let core: String = chars[lead_len..chars.len() - trail_len].iter().collect();
                let suffix: String = chars[chars.len() - trail_len..].iter().collect();

                let phones = langs::engine::phonemize_word(spec, &core, stage);
                format!("{}{}{}", prefix, phones, suffix)
            })
            .collect::<Vec<_>>()
            .join(" ");
    }
    match &*lang {
        "de" | "german" => text
            .split_whitespace()
            .map(|tok| {
                let chars: Vec<char> = tok.chars().collect();
                let lead_len = chars.iter().take_while(|&&c| !c.is_alphanumeric()).count();
                if lead_len == chars.len() {
                    return tok.to_string();
                }
                let trail_len = chars
                    .iter()
                    .rev()
                    .take_while(|&&c| !c.is_alphanumeric())
                    .count();
                let prefix: String = chars[..lead_len].iter().collect();
                let core: String = chars[lead_len..chars.len() - trail_len].iter().collect();
                let suffix: String = chars[chars.len() - trail_len..].iter().collect();

                let phones = langs::de::phonemize_de_word(&core);
                format!("{}{}{}", prefix, phones, suffix)
            })
            .collect::<Vec<_>>()
            .join(" "),
        _ => phonemize(text),
    }
}

/// Base-mapper segmentation of a word for a distilled language (see `langs::engine`).
pub fn base_segments(word: &str, lang: &str) -> Option<Vec<(String, String)>> {
    langs::spec(&lang.to_lowercase()).map(|spec| langs::engine::base_segments(spec, word))
}

/// Phonemize an entire text utterance with default configuration.
pub fn phonemize(text: &str) -> String {
    phonemize_with_config(text, &Config::default())
}

#[derive(Debug, Clone)]
enum TokenContent {
    Word(String),
    Literal(String),
}

#[derive(Debug, Clone)]
struct UtteranceToken {
    prefix_punct: String,
    content: TokenContent,
    suffix_punct: String,
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

    let mut tokens: Vec<UtteranceToken> = Vec::new();

    for raw_token in text.split_whitespace() {
        let chars: Vec<char> = raw_token.chars().collect();
        let lead_len = chars.iter().take_while(|&&c| !c.is_alphanumeric()).count();
        if lead_len == chars.len() {
            tokens.push(UtteranceToken {
                prefix_punct: String::new(),
                content: TokenContent::Literal(raw_token.to_string()),
                suffix_punct: String::new(),
            });
            continue;
        }

        let trail_len = chars
            .iter()
            .rev()
            .take_while(|&&c| !c.is_alphanumeric())
            .count();
        let prefix_punct: String = chars[..lead_len].iter().collect();
        let core: String = chars[lead_len..chars.len() - trail_len].iter().collect();
        let suffix_punct: String = chars[chars.len() - trail_len..].iter().collect();

        // If core contains digits, preserve as literal rather than silently dropping!
        let content = if core.chars().any(|c| c.is_ascii_digit()) {
            TokenContent::Literal(core)
        } else {
            TokenContent::Word(core)
        };

        tokens.push(UtteranceToken {
            prefix_punct,
            content,
            suffix_punct,
        });
    }

    if tokens.is_empty() {
        return String::new();
    }

    // Phonemize words
    let mut words_phonemes: Vec<Vec<Phoneme>> = Vec::with_capacity(tokens.len());
    let mut has_trailing_punct: Vec<bool> = Vec::with_capacity(tokens.len());

    for (i, tok) in tokens.iter().enumerate() {
        match &tok.content {
            TokenContent::Word(w) => {
                let phonemes = phonemize_word(w, config);
                words_phonemes.push(phonemes);
                let is_barrier = !tok.suffix_punct.is_empty()
                    || tokens.get(i + 1).is_none_or(|next| {
                        !next.prefix_punct.is_empty()
                            || matches!(next.content, TokenContent::Literal(_))
                    });
                has_trailing_punct.push(is_barrier);
            }
            TokenContent::Literal(_) => {
                words_phonemes.push(Vec::new());
                has_trailing_punct.push(true);
            }
        }
    }

    // Apply Sandhi across word boundaries
    if config.apply_sandhi {
        apply_sandhi(&mut words_phonemes, &has_trailing_punct, config);
    }

    // Assemble rendered output string
    let mut output = String::new();
    let num_tokens = tokens.len();

    for (i, tok) in tokens.iter().enumerate() {
        output.push_str(&tok.prefix_punct);
        match &tok.content {
            TokenContent::Word(_) => {
                for p in &words_phonemes[i] {
                    output.push_str(p.render(config.format));
                }
            }
            TokenContent::Literal(lit) => {
                output.push_str(lit);
            }
        }
        output.push_str(&tok.suffix_punct);

        if i + 1 < num_tokens {
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
        assert_eq!(
            phonemize_with_config("brat ojca", &config_warsaw),
            "brat ɔjt͡sa"
        );
        assert_eq!(
            phonemize_with_config("brat ojca", &config_krakow),
            "brad ɔjt͡sa"
        );

        // Vowel-final words must NEVER have internal consonants modified by sandhi!
        // "kącie, a" -> [kɔɲt͡ɕɛ, a] (NEVER *kɔɲd͡ʑɛ)
        assert_eq!(
            phonemize_with_config("przy kącie, a", &config_krakow),
            "pʂɨ kɔɲt͡ɕɛ, a"
        );
        assert_eq!(phonemize_with_config("kącie i", &config_krakow), "kɔɲt͡ɕɛ i");

        // Punctuation blocks sandhi:
        assert_eq!(
            phonemize_with_config("brat, ojca", &config_krakow),
            "brat, ɔjt͡sa"
        );
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
    fn test_silesian_nasal_decomposition() {
        let config = Config {
            dialect: Dialect::Slask,
            format: OutputFormat::IpaNarrow,
            include_stress: false,
            ..Default::default()
        };
        // Word-final -ą -> "-ōm" (nosówki rozłożone)
        assert_eq!(phonemize_with_config("idą", &config), "idom");
        assert_eq!(phonemize_with_config("są", &config), "som");
        // Before stops: homorganic nasal + raised nucleus ("piōntek", "kōmpiel")
        assert_eq!(phonemize_with_config("piątek", &config), "pʲontɛk");
        assert_eq!(phonemize_with_config("kąpiel", &config), "kompʲɛl");
        // Before fricatives: consonantal nasality [n] ("Ślōnsk")
        assert_eq!(phonemize_with_config("mąż", &config), "monʂ");
        assert_eq!(phonemize_with_config("chrząszcz", &config), "xʂonʂt͡ʂ");
        // ę-nucleus stays plain [ɛ]
        assert_eq!(phonemize_with_config("zęby", &config), "zɛmbɨ");
        assert_eq!(phonemize_with_config("ręka", &config), "rɛŋka");
        // Before liquids: bare nucleus ("wziōł")
        assert_eq!(phonemize_with_config("wziął", &config), "vʑow");
    }

    #[test]
    fn test_silesian_nasal_raising_toggle() {
        let config = Config {
            dialect: Dialect::Slask,
            format: OutputFormat::IpaNarrow,
            include_stress: false,
            silesian_raising: false,
            ..Default::default()
        };
        // Without the raising stage the ą-nucleus stays [ɔ]
        assert_eq!(phonemize_with_config("idą", &config), "idɔm");
        assert_eq!(phonemize_with_config("piątek", &config), "pʲɔntɛk");
    }

    #[test]
    fn test_silesian_region_final_e() {
        let base = Config {
            dialect: Dialect::Slask,
            format: OutputFormat::IpaNarrow,
            include_stress: false,
            ..Default::default()
        };
        // Word-final -ę reflexes: Katowice [a], Opole [ã], Cieszyn [e]
        let katowice = base.clone();
        assert_eq!(phonemize_with_config("idę", &katowice), "ida");
        let opole = Config {
            silesian_region: SilesianRegion::Opole,
            ..base.clone()
        };
        assert_eq!(phonemize_with_config("idę", &opole), "idã");
        let cieszyn = Config {
            silesian_region: SilesianRegion::Cieszyn,
            ..base
        };
        assert_eq!(phonemize_with_config("idę", &cieszyn), "idɛ");
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
        assert_eq!(phonemize_with_config("dubbing", &config), "dabʲiŋk");
        assert_eq!(phonemize_with_config("dubbingu", &config), "dabʲiŋɡu");
        assert_eq!(
            phonemize_with_config("zdubbingować", &config),
            "zdabʲiŋɡɔvat͡ɕ"
        );

        // 3. Prefix boundary dzi
        assert_eq!(phonemize_with_config("nadzieja", &config), "nad͡ʑɛja");
        assert_eq!(phonemize_with_config("podział", &config), "pɔd͡ʑaw");
        assert_eq!(phonemize_with_config("odzież", &config), "ɔd͡ʑɛʂ");

        // 4. Native -unia vs Latin
        assert_eq!(phonemize_with_config("babunia", &config), "babuɲa");
        assert_eq!(phonemize_with_config("unia", &config), "uɲja");

        // 5. Numeral compound simplification
        assert_eq!(
            phonemize_with_config("pięćdziesiąt", &config),
            "pʲɛɲd͡ʑɛɕɔnt"
        );
        assert_eq!(
            phonemize_with_config("pięćdziesiąta", &config),
            "pʲɛɲt͡ɕd͡ʑɛɕɔnta"
        );
        assert_eq!(
            phonemize_with_config("sześćdziesiąt", &config),
            "ʂɛʑd͡ʑɛɕɔnt"
        );
        assert_eq!(
            phonemize_with_config("sześćdziesiąta", &config),
            "ʂɛɕt͡ɕd͡ʑɛɕɔnta"
        );

        // 6. Sanskrit bh-
        assert_eq!(phonemize_with_config("bhutan", &config), "butan");

        // 7. Prefix bez- before i
        assert_eq!(phonemize_with_config("bezimienny", &config), "bɛzimʲɛnnɨ");
    }

    #[test]
    fn test_sandhi_spreads_through_final_cluster() {
        let krakow = Config {
            dialect: Dialect::KrakowPoznan,
            format: OutputFormat::IpaNarrow,
            include_stress: false,
            ..Default::default()
        };
        let warsaw = Config {
            dialect: Dialect::Warsaw,
            ..krakow.clone()
        };
        // Regressive voicing reaches the whole final cluster.
        assert_eq!(phonemize_with_config("jest on", &krakow), "jɛzd ɔn");
        assert_eq!(phonemize_with_config("jest dobrze", &krakow), "jɛzd dɔbʐɛ");
        assert_eq!(phonemize_with_config("jest dobrze", &warsaw), "jɛzd dɔbʐɛ");
        // Warsaw keeps the cluster voiceless before a vowel.
        assert_eq!(phonemize_with_config("jest on", &warsaw), "jɛst ɔn");
        // Sonorants block the spread (they stay in the coda).
        assert_eq!(phonemize_with_config("karp idzie", &krakow), "karb id͡ʑɛ");
    }

    #[test]
    fn test_w_devoicing_through_sonorants() {
        let config = Config {
            format: OutputFormat::IpaNarrow,
            include_stress: false,
            ..Default::default()
        };
        assert_eq!(phonemize_with_config("krwawy", &config), "krfavɨ");
        assert_eq!(phonemize_with_config("trwać", &config), "trfat͡ɕ");
        assert_eq!(phonemize_with_config("utrwalać", &config), "utrfalat͡ɕ");
        // A voiced obstruent leaves [v] intact, and a vowel blocks the search.
        assert_eq!(phonemize_with_config("drwal", &config), "drval");
        assert_eq!(phonemize_with_config("sowa", &config), "sɔva");
    }

    #[test]
    fn test_numbers_and_punctuation_preservation() {
        let config = Config {
            format: OutputFormat::IpaNarrow,
            include_stress: true,
            ..Default::default()
        };
        // Digits are preserved and not silently dropped
        assert_eq!(phonemize_with_config("rok 2024", &config), "ˈrɔk 2024");
        // Leading and trailing punctuation preserved
        assert_eq!(
            phonemize_with_config("„Dzień dobry!”", &config),
            "„ˈd͡ʑɛɲ ˈdɔbrɨ!”"
        );
        // Multilingual punctuation stripping for exception lookup
        let cs_out = phonemize_lang("chleba.", "cs");
        assert!(
            cs_out.ends_with('.'),
            "Expected trailing dot in cs_out, got: {cs_out}"
        );
        assert_eq!(cs_out, "xlɛba.");
    }

    #[test]
    fn test_velar_glide_only_before_front_vowels() {
        let config = Config {
            format: OutputFormat::IpaNarrow,
            include_stress: false,
            ..Default::default()
        };
        assert_eq!(phonemize_with_config("kiosk", &config), "kʲɔsk");
        assert_eq!(phonemize_with_config("giąć", &config), "ɡʲɔɲt͡ɕ");
        // Learned -ia/-ie keeps the glide.
        assert_eq!(phonemize_with_config("alergia", &config), "alɛrɡʲja");
        assert_eq!(phonemize_with_config("celiakia", &config), "t͡sɛlʲakʲja");
        assert_eq!(phonemize_with_config("hiena", &config), "xʲjɛna");
    }

    #[test]
    fn test_prefix_after_nie_and_diphthong_respellings() {
        let config = Config {
            format: OutputFormat::IpaNarrow,
            include_stress: false,
            ..Default::default()
        };
        assert_eq!(phonemize_with_config("nieodzowny", &config), "ɲɛɔdzɔvnɨ");
        assert_eq!(
            phonemize_with_config("zautomatyzować", &config),
            "zawtɔmatɨzɔvat͡ɕ"
        );
        assert_eq!(phonemize_with_config("nautyka", &config), "nawtɨka");
        // Established hiatus cases stay unchanged.
        assert_eq!(phonemize_with_config("nauka", &config), "nauka");
        assert_eq!(phonemize_with_config("zaufać", &config), "zaufat͡ɕ");
    }

    #[test]
    fn test_stress_mark_placement_in_clusters() {
        let config = Config {
            format: OutputFormat::IpaNarrow,
            include_stress: true,
            ..Default::default()
        };
        assert_eq!(phonemize_with_config("Warszawa", &config), "varˈʂava");
        assert_eq!(
            phonemize_with_config("czternaście", &config),
            "t͡ʂtɛrˈnaɕt͡ɕɛ"
        );
        assert_eq!(phonemize_with_config("komnata", &config), "kɔmˈnata");
        assert_eq!(phonemize_with_config("matematyka", &config), "matɛˈmatɨka");
        assert_eq!(phonemize_with_config("brat", &config), "ˈbrat");
    }

    #[test]
    fn test_nasal_place_assimilation_before_velars() {
        let config = Config {
            format: OutputFormat::IpaNarrow,
            include_stress: false,
            ..Default::default()
        };
        assert_eq!(phonemize_with_config("inkub", &config), "iŋkup");
        assert_eq!(phonemize_with_config("kolorowanka", &config), "kɔlɔrɔvaŋka");
        assert_eq!(phonemize_with_config("bank", &config), "baŋk");
    }

    #[test]
    fn test_output_formats_keep_affricate_cluster_contrast() {
        for format in [
            OutputFormat::IpaNarrow,
            OutputFormat::IpaBroad,
            OutputFormat::Sampa,
            OutputFormat::Piper,
            OutputFormat::Benni,
        ] {
            let config = Config {
                format,
                include_stress: false,
                ..Default::default()
            };
            assert_ne!(
                phonemize_with_config("czysta", &config),
                phonemize_with_config("trzysta", &config),
                "format {format:?} merges czysta and trzysta"
            );
        }
    }
}
