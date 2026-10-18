//! Sentence-level Sandhi (fonetyka międzywyrazowa) across word boundaries.

use crate::phoneme::Phoneme;
use crate::types::{Config, Dialect};

/// Apply inter-word sandhi across tokens in a sentence.
pub fn apply_sandhi(
    words_phonemes: &mut Vec<Vec<Phoneme>>,
    has_trailing_punct: &[bool],
    config: &Config,
) {
    if !config.apply_sandhi || words_phonemes.len() < 2 {
        return;
    }

    for i in 0..words_phonemes.len() - 1 {
        // Sandhi NEVER occurs across punctuation / syntactic pauses (e.g. comma, period, semicolon)
        if has_trailing_punct.get(i).copied().unwrap_or(false) {
            continue;
        }

        // Find last phoneme of word i and first phoneme of word i + 1
        let (left_part, right_part) = words_phonemes.split_at_mut(i + 1);
        let word_a = &mut left_part[i];
        let word_b = &right_part[0];

        // Find the absolute last audible phoneme in word_a (ignoring only trailing stress/break markers)
        let last_idx = word_a.iter().rposition(|p| {
            !matches!(
                p,
                Phoneme::PrimaryStress
                    | Phoneme::SecondaryStress
                    | Phoneme::SyllableBreak
                    | Phoneme::WordBreak
                    | Phoneme::SandhiTie
            )
        });

        // Find first audible phoneme in word_b
        let first_idx = word_b.iter().position(|p| {
            !matches!(
                p,
                Phoneme::PrimaryStress
                    | Phoneme::SecondaryStress
                    | Phoneme::SyllableBreak
                    | Phoneme::WordBreak
                    | Phoneme::SandhiTie
            )
        });

        if let (Some(la_idx), Some(fb_idx)) = (last_idx, first_idx) {
            let last_p = word_a[la_idx];
            let first_p = word_b[fb_idx];

            // Crucial: Sandhi across word boundaries ONLY applies to word-FINAL obstruents!
            // If the word ends in a vowel (e.g. "kącie" -> [kɔɲt͡ɕɛ]) or sonorant,
            // internal consonants inside the word must NEVER be modified by sandhi!
            if !last_p.is_obstruent() {
                continue;
            }

            // Rule 1: Before voiced obstruents (excluding /v/), voicing applies everywhere in Poland
            if first_p.is_obstruent()
                && first_p.is_voiced()
                && first_p != Phoneme::V
                && first_p != Phoneme::VPal
            {
                if !last_p.is_voiced() {
                    word_a[la_idx] = last_p.to_voiced();
                }
            }
            // Rule 2: Before voiceless obstruents, devoicing applies everywhere
            else if first_p.is_obstruent() && !first_p.is_voiced() {
                if last_p.is_voiced() {
                    word_a[la_idx] = last_p.to_devoiced();
                }
            }
            // Rule 3: Before vowels and sonorants (m, n, ń, r, l, ł, j)
            else if first_p.is_sonorant() {
                match config.dialect {
                    Dialect::KrakowPoznan | Dialect::Slask => {
                        // Krakow-Poznan & Silesia: Sandhi voicing (udźwięcznienie międzywyrazowe)
                        // "brat ojca" -> [brad‿ɔjt͡sa]
                        // "las rośnie" -> [laz‿rɔɕɲɛ]
                        if !last_p.is_voiced() {
                            word_a[la_idx] = last_p.to_voiced();
                        }
                    }
                    Dialect::Warsaw => {
                        // Warsaw / Standard Polish: Devoicing before vowels and sonorants (ubezdźwięcznienie)
                        // "brat ojca" -> [brat‿ɔjt͡sa]
                        // "las rośnie" -> [las‿rɔɕɲɛ]
                        if last_p.is_voiced() {
                            word_a[la_idx] = last_p.to_devoiced();
                        }
                    }
                }
            }
        }
    }
}
