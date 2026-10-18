//! Core types and configuration for TBenni Polish Phonemizer.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Polish dialect / pronunciation variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Dialect {
    /// Standard Central/Eastern Polish (Warsaw, Mazovia, Podlasie).
    /// Features inter-word devoicing (fonetyka ubezdźwięczniająca):
    /// "brat ojca" -> [brat‿ɔjt͡sa].
    Warsaw,

    /// Standard Southern/Western Polish (Cracow, Poznań, Małopolska, Wielkopolska).
    /// Features inter-word voicing (fonetyka udźwięczniająca):
    /// "brat ojca" -> [brad‿ɔjt͡sa].
    #[default]
    KrakowPoznan,

    /// Silesian ethnolect / dialect (Górny Śląsk / Ślōnskŏ gŏdka).
    /// Features:
    /// - Inter-word sandhi voicing (udźwięcznienie międzywyrazowe).
    /// - Asynchronous / decomposed nasal vowels (e.g. ą -> ōm/ōn, ę -> ym/yn, wygłos -ą -> -om/-um, -ę -> -a/-ym).
    /// - Initial /o-/ labialization (np. okno -> [wɔknɔ]).
    /// - Raised / slanted vowels (samogłoski pochylone).
    Slask,
}

/// Output phonetic alphabet / notation format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OutputFormat {
    /// Narrow International Phonetic Alphabet with allophonic diacritics,
    /// diphthongal glides, tie-bars, and stress marks (e.g. [ˈt͡ʂɨsta], [vɔw̃s], [dɔmp]).
    #[default]
    IpaNarrow,

    /// Broad phonemic IPA transcription (e.g. /ˈtʂɨsta/, /vɔws/, /dɔmp/).
    IpaBroad,

    /// Speech Assessment Methods Phonetic Alphabet (SAMPA / X-SAMPA).
    /// 7-bit ASCII representation for speech synthesizers (e.g. "tS'1sta", "dOmp").
    Sampa,

    /// Slavic Phonetic Alphabet (AS / Alfabet Slawistyczny), honoring Tytus Benni.
    /// Uses single-glyph affricates (č, ǯ, ć, ʒ́, c, ʒ) and Slavic diacritics (np. "č'ysta", "vǫs", "dǫp").
    Benni,

    /// Token string compatible with Piper TTS (pl_PL ONNX models).
    Piper,
}

/// Stress assignment mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StressMode {
    /// Prescriptive standard (norma wzorcowa).
    /// Antepenultimate stress on Greek/Latin borrowings (-yka/-ika: "fizyka", "matematyka"),
    /// 1st/2nd pl past tense verbs (-liśmy, -liście: "byliśmy"), and conditional forms (-byśmy: "zrobilibyśmy").
    #[default]
    Prescriptive,

    /// Modern colloquial / regularized (norma użytkowa).
    /// Generalizes penultimate stress across all words (e.g. "fizy'ka", "byli'śmy").
    Colloquial,
}

/// Phonemizer configuration options.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Config {
    /// Dialect / sandhi region.
    pub dialect: Dialect,
    /// Output transcription format.
    pub format: OutputFormat,
    /// Word stress mode.
    pub stress_mode: StressMode,
    /// Whether to insert stress marks (' or ˈ).
    pub include_stress: bool,
    /// Whether to insert syllable break markers (.).
    pub include_syllable_breaks: bool,
    /// Whether to apply inter-word sandhi across word boundaries.
    pub apply_sandhi: bool,
    /// Whether to denasalize word-final 'ę' to oral [ɛ] (true = cultured standard [idɛ], false = archaic/stage [idɛw̃]).
    pub denasalize_final_e: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dialect: Dialect::KrakowPoznan,
            format: OutputFormat::IpaNarrow,
            stress_mode: StressMode::Prescriptive,
            include_stress: true,
            include_syllable_breaks: false,
            apply_sandhi: true,
            denasalize_final_e: true,
        }
    }
}
