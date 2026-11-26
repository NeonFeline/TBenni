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
    #[default]
    Warsaw,

    /// Standard Southern/Western Polish (Cracow, Poznań, Małopolska, Wielkopolska).
    /// Features inter-word voicing (fonetyka udźwięczniająca):
    /// "brat ojca" -> [brad‿ɔjt͡sa].
    KrakowPoznan,

    /// Silesian ethnolect / dialect (Górny Śląsk / Ślōnskŏ gŏdka).
    /// Implemented features:
    /// - Inter-word sandhi voicing (udźwięcznienie międzywyrazowe).
    /// - Initial /o-/ labialization (np. okno -> [wɔknɔ]).
    /// - é (e pochylone) -> [ɨ].
    /// - Decomposed/asynchronous nasal vowels (nosówki rozłożone, Cieszyn norm
    ///   Zasada 6.6): ą/ę -> oral vowel + homorganic nasal consonant
    ///   (np. piątek -> [pjontɛk], są -> [som], mąż -> [monʐ]).
    ///   Surface vowel quality is decided by the raising stage afterwards
    ///   (Config::silesian_raising), and word-final -ę varies by
    ///   Config::silesian_region.
    ///
    /// Not yet implemented:
    /// - The full Silesian vowel-raising system (a/á -> o, ó -> u, é -> y).
    Slask,
}

/// Silesian sub-region, controlling regional reflexes (currently word-final -ę).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SilesianRegion {
    /// Industrial Upper Silesia (katowicki). Word-final -ę -> [a]
    /// (np. idę -> [ida]).
    #[default]
    Katowice,

    /// Opole Silesia (śląsk opolski). Word-final -ę -> [ã]
    /// ("a z tyldą", np. idę -> [idã]).
    Opole,

    /// Cieszyn Silesia (śląsk cieszyński). Word-final -e -> [e]
    /// (accusative realization). The 1st-sg verb reflex [-ɨm] is
    /// morphologically conditioned and not yet modeled.
    Cieszyn,
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
    /// Silesian sub-region for regional reflexes (word-final -ę); used only
    /// when `dialect == Dialect::Slask`.
    pub silesian_region: SilesianRegion,
    /// Silesian nasal-nucleus raising stage (applied after decomposition):
    /// when true, the nucleus of decomposed 'ą' raises to [o] ("idōm" -> [idom],
    /// "piątek" -> [pjontɛk]); when false it stays [ɔ] ([idɔm], [pjɔntɛk]).
    /// The nucleus of decomposed 'ę' is always plain [ɛ] (Cieszyn norm).
    pub silesian_raising: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dialect: Dialect::Warsaw,
            format: OutputFormat::IpaNarrow,
            stress_mode: StressMode::Prescriptive,
            include_stress: true,
            include_syllable_breaks: false,
            apply_sandhi: true,
            denasalize_final_e: true,
            silesian_region: SilesianRegion::default(),
            silesian_raising: true,
        }
    }
}
