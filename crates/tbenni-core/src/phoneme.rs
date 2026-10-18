//! Phoneme definitions and multi-alphabet transcription representations.

use crate::types::OutputFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phoneme {
    // Vowels
    A,
    E,
    I,
    Y, // [ɨ]
    O,
    U,

    // Silesian raised / slanted vowels
    OSlant, // Śl. ō / o pochylone [o] / [o̝] / [u]
    ESlant, // Śl. é / e pochylone [e] / [e̝] / [ɨ]
    ASlant, // Śl. á / a pochylone [ɔ]

    // Nasal Vowels / Glides
    ENasal, // [ɛw̃]
    ONasal, // [ɔw̃]

    // Plosives
    P,
    B,
    T,
    D,
    K,
    G,
    KPal, // [kʲ] / [c]
    GPal, // [ɡʲ] / [ɟ]

    // Palatalized Labials
    PPal, // [pʲ]
    BPal, // [bʲ]
    FPal, // [fʲ]
    VPal, // [vʲ]
    MPal, // [mʲ]

    // Affricates
    C,       // [t͡s] dental
    Dz,      // [d͡z] dental
    Cz,      // [t͡ʂ] postalveolar / retroflex
    DzRetro, // [d͡ʐ] postalveolar / retroflex
    Ci,      // [t͡ɕ] alveolo-palatal
    Dzi,     // [d͡ʑ] alveolo-palatal

    // Fricatives
    F,
    V,
    S,
    Z,
    Sz,     // [ʂ]
    ZRetro, // [ʐ] from ż
    Rz,     // [ʐ] from rz (undergoes progressive devoicing after voiceless consonants)
    Si,     // [ɕ]
    Zi,     // [ʑ]
    Ch,     // [x]
    ChPal,  // [xʲ]
    Gh,     // [ɣ] voiced velar fricative (allophone)

    // Nasals
    M,
    N,      // [n] dental
    Ni,     // [ɲ] alveolo-palatal
    NRetro, // [n̠] postalveolar
    Ng,     // [ŋ] velar

    // Liquids & Semivowels
    R,
    RPal, // [rʲ] palatalized trill in foreign borrowings (e.g. adrian, historia)
    L,
    LPal, // [lʲ]
    W,    // [w] (ł)
    J,    // [j]

    // Prosody & Delimiters
    PrimaryStress,
    SecondaryStress,
    SyllableBreak,
    WordBreak,
    SandhiTie,
    GlottalStop, // [ʔ]
    Punctuation(char),
}

impl Phoneme {
    /// Is this phoneme a vowel?
    pub fn is_vowel(&self) -> bool {
        matches!(
            self,
            Phoneme::A
                | Phoneme::E
                | Phoneme::I
                | Phoneme::Y
                | Phoneme::O
                | Phoneme::U
                | Phoneme::OSlant
                | Phoneme::ESlant
                | Phoneme::ASlant
                | Phoneme::ENasal
                | Phoneme::ONasal
        )
    }

    /// Is this phoneme an obstruent (plosive, affricate, fricative)?
    pub fn is_obstruent(&self) -> bool {
        matches!(
            self,
            Phoneme::P
                | Phoneme::B
                | Phoneme::T
                | Phoneme::D
                | Phoneme::K
                | Phoneme::G
                | Phoneme::KPal
                | Phoneme::GPal
                | Phoneme::PPal
                | Phoneme::BPal
                | Phoneme::FPal
                | Phoneme::VPal
                | Phoneme::C
                | Phoneme::Dz
                | Phoneme::Cz
                | Phoneme::DzRetro
                | Phoneme::Ci
                | Phoneme::Dzi
                | Phoneme::F
                | Phoneme::V
                | Phoneme::S
                | Phoneme::Z
                | Phoneme::Sz
                | Phoneme::ZRetro
                | Phoneme::Rz
                | Phoneme::Si
                | Phoneme::Zi
                | Phoneme::Ch
                | Phoneme::ChPal
                | Phoneme::Gh
        )
    }

    /// Is this phoneme voiced?
    pub fn is_voiced(&self) -> bool {
        if self.is_vowel() {
            return true;
        }
        matches!(
            self,
            Phoneme::B
                | Phoneme::D
                | Phoneme::G
                | Phoneme::GPal
                | Phoneme::BPal
                | Phoneme::VPal
                | Phoneme::Dz
                | Phoneme::DzRetro
                | Phoneme::Dzi
                | Phoneme::V
                | Phoneme::Z
                | Phoneme::ZRetro
                | Phoneme::Rz
                | Phoneme::Zi
                | Phoneme::Gh
                | Phoneme::M
                | Phoneme::MPal
                | Phoneme::N
                | Phoneme::Ni
                | Phoneme::NRetro
                | Phoneme::Ng
                | Phoneme::R
                | Phoneme::RPal
                | Phoneme::L
                | Phoneme::LPal
                | Phoneme::W
                | Phoneme::J
        )
    }

    /// Is this phoneme a sonorant (liquid, nasal, glide, vowel)?
    pub fn is_sonorant(&self) -> bool {
        self.is_vowel()
            || matches!(
                self,
                Phoneme::M
                    | Phoneme::MPal
                    | Phoneme::N
                    | Phoneme::Ni
                    | Phoneme::NRetro
                    | Phoneme::Ng
                    | Phoneme::R
                    | Phoneme::RPal
                    | Phoneme::L
                    | Phoneme::LPal
                    | Phoneme::W
                    | Phoneme::J
            )
    }

    /// Get devoiced counterpart of voiced obstruent.
    pub fn to_devoiced(&self) -> Self {
        match self {
            Phoneme::B => Phoneme::P,
            Phoneme::D => Phoneme::T,
            Phoneme::G => Phoneme::K,
            Phoneme::GPal => Phoneme::KPal,
            Phoneme::BPal => Phoneme::PPal,
            Phoneme::VPal => Phoneme::FPal,
            Phoneme::V => Phoneme::F,
            Phoneme::Z => Phoneme::S,
            Phoneme::ZRetro => Phoneme::Sz,
            Phoneme::Rz => Phoneme::Sz,
            Phoneme::Zi => Phoneme::Si,
            Phoneme::Dz => Phoneme::C,
            Phoneme::DzRetro => Phoneme::Cz,
            Phoneme::Dzi => Phoneme::Ci,
            Phoneme::Gh => Phoneme::Ch,
            other => *other,
        }
    }

    /// Get voiced counterpart of voiceless obstruent.
    pub fn to_voiced(&self) -> Self {
        match self {
            Phoneme::P => Phoneme::B,
            Phoneme::T => Phoneme::D,
            Phoneme::K => Phoneme::G,
            Phoneme::KPal => Phoneme::GPal,
            Phoneme::PPal => Phoneme::BPal,
            Phoneme::FPal => Phoneme::VPal,
            Phoneme::F => Phoneme::V,
            Phoneme::S => Phoneme::Z,
            Phoneme::Sz => Phoneme::ZRetro,
            Phoneme::Si => Phoneme::Zi,
            Phoneme::C => Phoneme::Dz,
            Phoneme::Cz => Phoneme::DzRetro,
            Phoneme::Ci => Phoneme::Dzi,
            Phoneme::Ch => Phoneme::Gh,
            Phoneme::ChPal => Phoneme::GPal,
            other => *other,
        }
    }

    /// Render phoneme to string according to requested format.
    pub fn render(&self, format: OutputFormat) -> &'static str {
        match format {
            OutputFormat::IpaNarrow => self.to_ipa_narrow(),
            OutputFormat::IpaBroad => self.to_ipa_broad(),
            OutputFormat::Sampa => self.to_sampa(),
            OutputFormat::Benni => self.to_benni(),
            OutputFormat::Piper => self.to_piper(),
        }
    }

    pub fn to_ipa_narrow(&self) -> &'static str {
        match self {
            Phoneme::A => "a",
            Phoneme::E => "ɛ",
            Phoneme::I => "i",
            Phoneme::Y => "ɨ",
            Phoneme::O => "ɔ",
            Phoneme::U => "u",
            Phoneme::OSlant => "o",
            Phoneme::ESlant => "e",
            Phoneme::ASlant => "ɔ",
            Phoneme::ENasal => "ɛw̃",
            Phoneme::ONasal => "ɔw̃",
            Phoneme::P => "p",
            Phoneme::B => "b",
            Phoneme::T => "t",
            Phoneme::D => "d",
            Phoneme::K => "k",
            Phoneme::G => "ɡ",
            Phoneme::KPal => "kʲ",
            Phoneme::GPal => "ɡʲ",
            Phoneme::PPal => "pʲ",
            Phoneme::BPal => "bʲ",
            Phoneme::FPal => "fʲ",
            Phoneme::VPal => "vʲ",
            Phoneme::MPal => "mʲ",
            Phoneme::C => "t͡s",
            Phoneme::Dz => "d͡z",
            Phoneme::Cz => "t͡ʂ",
            Phoneme::DzRetro => "d͡ʐ",
            Phoneme::Ci => "t͡ɕ",
            Phoneme::Dzi => "d͡ʑ",
            Phoneme::F => "f",
            Phoneme::V => "v",
            Phoneme::S => "s",
            Phoneme::Z => "z",
            Phoneme::Sz => "ʂ",
            Phoneme::ZRetro => "ʐ",
            Phoneme::Rz => "ʐ",
            Phoneme::Si => "ɕ",
            Phoneme::Zi => "ʑ",
            Phoneme::Ch => "x",
            Phoneme::ChPal => "xʲ",
            Phoneme::Gh => "ɣ",
            Phoneme::M => "m",
            Phoneme::N => "n",
            Phoneme::Ni => "ɲ",
            Phoneme::NRetro => "n̠",
            Phoneme::Ng => "ŋ",
            Phoneme::R => "r",
            Phoneme::RPal => "rʲ",
            Phoneme::L => "l",
            Phoneme::LPal => "lʲ",
            Phoneme::W => "w",
            Phoneme::J => "j",
            Phoneme::PrimaryStress => "ˈ",
            Phoneme::SecondaryStress => "ˌ",
            Phoneme::SyllableBreak => ".",
            Phoneme::WordBreak => " ",
            Phoneme::SandhiTie => "‿",
            Phoneme::GlottalStop => "ʔ",
            Phoneme::Punctuation(c) => match c {
                '.' => ".",
                ',' => ",",
                '!' => "!",
                '?' => "?",
                ':' => ":",
                ';' => ";",
                '-' => "-",
                _ => "",
            },
        }
    }

    pub fn to_ipa_broad(&self) -> &'static str {
        match self {
            Phoneme::A => "a",
            Phoneme::E => "ɛ",
            Phoneme::I => "i",
            Phoneme::Y => "ɨ",
            Phoneme::O => "ɔ",
            Phoneme::U => "u",
            Phoneme::OSlant => "o",
            Phoneme::ESlant => "e",
            Phoneme::ASlant => "ɔ",
            Phoneme::ENasal => "ɛ̃",
            Phoneme::ONasal => "ɔ̃",
            Phoneme::P => "p",
            Phoneme::B => "b",
            Phoneme::T => "t",
            Phoneme::D => "d",
            Phoneme::K => "k",
            Phoneme::G => "ɡ",
            Phoneme::KPal => "kʲ",
            Phoneme::GPal => "ɡʲ",
            Phoneme::PPal => "pʲ",
            Phoneme::BPal => "bʲ",
            Phoneme::FPal => "fʲ",
            Phoneme::VPal => "vʲ",
            Phoneme::MPal => "mʲ",
            Phoneme::C => "t͡s",
            Phoneme::Dz => "d͡z",
            Phoneme::Cz => "t͡ʂ",
            Phoneme::DzRetro => "d͡ʐ",
            Phoneme::Ci => "t͡ɕ",
            Phoneme::Dzi => "d͡ʑ",
            Phoneme::F => "f",
            Phoneme::V => "v",
            Phoneme::S => "s",
            Phoneme::Z => "z",
            Phoneme::Sz => "ʂ",
            Phoneme::ZRetro => "ʐ",
            Phoneme::Rz => "ʐ",
            Phoneme::Si => "ɕ",
            Phoneme::Zi => "ʑ",
            Phoneme::Ch => "x",
            Phoneme::ChPal => "xʲ",
            Phoneme::Gh => "ɣ",
            Phoneme::M => "m",
            Phoneme::N => "n",
            Phoneme::Ni => "ɲ",
            Phoneme::NRetro => "n",
            Phoneme::Ng => "ŋ",
            Phoneme::R => "r",
            Phoneme::RPal => "r",
            Phoneme::L => "l",
            Phoneme::LPal => "lʲ",
            Phoneme::W => "w",
            Phoneme::J => "j",
            Phoneme::PrimaryStress => "ˈ",
            Phoneme::SecondaryStress => "ˌ",
            Phoneme::SyllableBreak => ".",
            Phoneme::WordBreak => " ",
            Phoneme::SandhiTie => " ",
            Phoneme::GlottalStop => "ʔ",
            Phoneme::Punctuation(c) => match c {
                '.' => ".",
                ',' => ",",
                '!' => "!",
                '?' => "?",
                _ => "",
            },
        }
    }

    pub fn to_benni(&self) -> &'static str {
        match self {
            Phoneme::A => "a",
            Phoneme::E => "e",
            Phoneme::I => "i",
            Phoneme::Y => "y",
            Phoneme::O => "o",
            Phoneme::U => "u",
            Phoneme::OSlant => "ó",
            Phoneme::ESlant => "é",
            Phoneme::ASlant => "á",
            Phoneme::ENasal => "ę",
            Phoneme::ONasal => "ǫ",
            Phoneme::P => "p",
            Phoneme::B => "b",
            Phoneme::T => "t",
            Phoneme::D => "d",
            Phoneme::K => "k",
            Phoneme::G => "g",
            Phoneme::KPal => "k'",
            Phoneme::GPal => "g'",
            Phoneme::PPal => "p'",
            Phoneme::BPal => "b'",
            Phoneme::FPal => "f'",
            Phoneme::VPal => "v'",
            Phoneme::MPal => "m'",
            Phoneme::C => "c",
            Phoneme::Dz => "ʒ",
            Phoneme::Cz => "č",
            Phoneme::DzRetro => "ǯ",
            Phoneme::Ci => "ć",
            Phoneme::Dzi => "ʒ́",
            Phoneme::F => "f",
            Phoneme::V => "v",
            Phoneme::S => "s",
            Phoneme::Z => "z",
            Phoneme::Sz => "š",
            Phoneme::ZRetro => "ž",
            Phoneme::Rz => "ž",
            Phoneme::Si => "ś",
            Phoneme::Zi => "ź",
            Phoneme::Ch => "x",
            Phoneme::ChPal => "x'",
            Phoneme::Gh => "γ",
            Phoneme::M => "m",
            Phoneme::N => "n",
            Phoneme::Ni => "ń",
            Phoneme::NRetro => "n̠",
            Phoneme::Ng => "ŋ",
            Phoneme::R => "r",
            Phoneme::RPal => "r’",
            Phoneme::L => "l",
            Phoneme::LPal => "l'",
            Phoneme::W => "u̯",
            Phoneme::J => "j",
            Phoneme::PrimaryStress => "'",
            Phoneme::SecondaryStress => "`",
            Phoneme::SyllableBreak => ".",
            Phoneme::WordBreak => " ",
            Phoneme::SandhiTie => "‿",
            Phoneme::GlottalStop => "ʔ",
            Phoneme::Punctuation(c) => match c {
                '.' => ".",
                ',' => ",",
                '!' => "!",
                '?' => "?",
                _ => "",
            },
        }
    }

    pub fn to_sampa(&self) -> &'static str {
        match self {
            Phoneme::A => "a",
            Phoneme::E => "e",
            Phoneme::I => "i",
            Phoneme::Y => "1",
            Phoneme::O => "o",
            Phoneme::U => "u",
            Phoneme::OSlant => "o",
            Phoneme::ESlant => "e",
            Phoneme::ASlant => "o",
            Phoneme::ENasal => "e~",
            Phoneme::ONasal => "o~",
            Phoneme::P => "p",
            Phoneme::B => "b",
            Phoneme::T => "t",
            Phoneme::D => "d",
            Phoneme::K => "k",
            Phoneme::G => "g",
            Phoneme::KPal => "k'",
            Phoneme::GPal => "g'",
            Phoneme::PPal => "p'",
            Phoneme::BPal => "b'",
            Phoneme::FPal => "f'",
            Phoneme::VPal => "v'",
            Phoneme::MPal => "m'",
            Phoneme::C => "t_s",
            Phoneme::Dz => "d_z",
            Phoneme::Cz => "t_S",
            Phoneme::DzRetro => "d_Z",
            Phoneme::Ci => "t_s\\",
            Phoneme::Dzi => "d_z\\",
            Phoneme::F => "f",
            Phoneme::V => "v",
            Phoneme::S => "s",
            Phoneme::Z => "z",
            Phoneme::Sz => "S",
            Phoneme::ZRetro => "Z",
            Phoneme::Rz => "Z",
            Phoneme::Si => "s\\",
            Phoneme::Zi => "z\\",
            Phoneme::Ch => "x",
            Phoneme::ChPal => "x'",
            Phoneme::Gh => "G",
            Phoneme::M => "m",
            Phoneme::N => "n",
            Phoneme::Ni => "n'",
            Phoneme::NRetro => "n",
            Phoneme::Ng => "N",
            Phoneme::R => "r",
            Phoneme::RPal => "r'",
            Phoneme::L => "l",
            Phoneme::LPal => "l'",
            Phoneme::W => "w",
            Phoneme::J => "j",
            Phoneme::PrimaryStress => "\"",
            Phoneme::SecondaryStress => "%",
            Phoneme::SyllableBreak => ".",
            Phoneme::WordBreak => " ",
            Phoneme::SandhiTie => "_",
            Phoneme::GlottalStop => "?",
            Phoneme::Punctuation(c) => match c {
                '.' => ".",
                ',' => ",",
                '!' => "!",
                '?' => "?",
                _ => "",
            },
        }
    }

    pub fn to_piper(&self) -> &'static str {
        // Piper's phoneme_id_map is character-based (no tie bars). Affricates are
        // written as stop + fricative; they use the retroflex fricative (tʂ, dʐ) so
        // they stay distinct from real clusters (trzysta -> tʃ), while sz/ż keep
        // eSpeak's palato-alveolar symbols.
        match self {
            Phoneme::A => "a",
            Phoneme::E => "ɛ",
            Phoneme::I => "i",
            Phoneme::Y => "ɨ",
            Phoneme::O => "ɔ",
            Phoneme::U => "u",
            Phoneme::OSlant => "o",
            Phoneme::ESlant => "e",
            Phoneme::ASlant => "ɔ",
            Phoneme::ENasal => "ɛ\u{0303}",
            Phoneme::ONasal => "ɔ\u{0303}",
            Phoneme::P => "p",
            Phoneme::B => "b",
            Phoneme::T => "t",
            Phoneme::D => "d",
            Phoneme::K => "k",
            Phoneme::G => "ɡ",
            Phoneme::KPal => "k\u{02B2}",
            Phoneme::GPal => "ɡ\u{02B2}",
            Phoneme::PPal => "p\u{02B2}",
            Phoneme::BPal => "b\u{02B2}",
            Phoneme::FPal => "f\u{02B2}",
            Phoneme::VPal => "v\u{02B2}",
            Phoneme::MPal => "m\u{02B2}",
            Phoneme::C => "ts",
            Phoneme::Dz => "dz",
            Phoneme::Cz => "tʂ",
            Phoneme::DzRetro => "dʐ",
            Phoneme::Ci => "tɕ",
            Phoneme::Dzi => "dʑ",
            Phoneme::F => "f",
            Phoneme::V => "v",
            Phoneme::S => "s",
            Phoneme::Z => "z",
            Phoneme::Sz => "ʃ",
            Phoneme::ZRetro => "ʒ",
            Phoneme::Rz => "ʒ",
            Phoneme::Si => "ɕ",
            Phoneme::Zi => "ʑ",
            Phoneme::Ch => "x",
            Phoneme::ChPal => "x\u{02B2}",
            Phoneme::Gh => "ɣ",
            Phoneme::M => "m",
            Phoneme::N => "n",
            Phoneme::Ni => "ɲ",
            Phoneme::NRetro => "n",
            Phoneme::Ng => "ŋ",
            Phoneme::R => "r",
            Phoneme::RPal => "r\u{02B2}",
            Phoneme::L => "l",
            Phoneme::LPal => "l\u{02B2}",
            Phoneme::W => "w",
            Phoneme::J => "j",
            Phoneme::PrimaryStress => "ˈ",
            Phoneme::SecondaryStress => "ˌ",
            Phoneme::SyllableBreak => "",
            Phoneme::WordBreak => " ",
            Phoneme::SandhiTie => " ",
            Phoneme::GlottalStop => "ʔ",
            Phoneme::Punctuation(c) => match c {
                '.' => ".",
                ',' => ",",
                '!' => "!",
                '?' => "?",
                _ => "",
            },
        }
    }
}
