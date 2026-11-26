#![allow(clippy::useless_conversion)]

use pyo3::prelude::*;
use rayon::prelude::*;
use tbenni_core::langs::engine::Stage;
use tbenni_core::{
    phonemize_with_config, Config, Dialect as CoreDialect, OutputFormat as CoreOutputFormat,
    SilesianRegion as CoreSilesianRegion, StressMode as CoreStressMode,
};

fn build_config(
    dialect: &str,
    format: &str,
    stress: &str,
    include_stress: bool,
    apply_sandhi: bool,
    region: &str,
    silesian_raising: bool,
) -> Config {
    let d = match dialect.to_lowercase().as_str() {
        "warsaw" | "warszawa" | "mazovia" | "standard" => CoreDialect::Warsaw,
        "krakow" | "krakow_poznan" | "poznan" => CoreDialect::KrakowPoznan,
        "slask" | "silesian" | "silesia" | "slonsk" => CoreDialect::Slask,
        _ => CoreDialect::Warsaw,
    };

    let f = match format.to_lowercase().as_str() {
        "ipa" | "ipa_narrow" | "narrow" => CoreOutputFormat::IpaNarrow,
        "ipa_broad" | "broad" => CoreOutputFormat::IpaBroad,
        "sampa" | "x_sampa" | "xsampa" => CoreOutputFormat::Sampa,
        "benni" | "as" | "slavic" => CoreOutputFormat::Benni,
        "piper" | "piper_tts" => CoreOutputFormat::Piper,
        _ => CoreOutputFormat::IpaNarrow,
    };

    let s = match stress.to_lowercase().as_str() {
        "prescriptive" | "standard" => CoreStressMode::Prescriptive,
        "colloquial" | "potoczny" => CoreStressMode::Colloquial,
        _ => CoreStressMode::Prescriptive,
    };

    let r = match region.to_lowercase().as_str() {
        "opole" | "opolski" => CoreSilesianRegion::Opole,
        "cieszyn" | "cieszyinski" | "cieszyński" => CoreSilesianRegion::Cieszyn,
        _ => CoreSilesianRegion::Katowice,
    };

    Config {
        dialect: d,
        format: f,
        stress_mode: s,
        include_stress,
        apply_sandhi,
        silesian_region: r,
        silesian_raising,
        ..Default::default()
    }
}

#[pyfunction]
#[pyo3(signature = (text, dialect = "warsaw", format = "ipa", stress = "prescriptive", include_stress = true, apply_sandhi = true, region = "katowice", silesian_raising = true))]
#[allow(clippy::too_many_arguments)]
fn phonemize(
    text: &str,
    dialect: &str,
    format: &str,
    stress: &str,
    include_stress: bool,
    apply_sandhi: bool,
    region: &str,
    silesian_raising: bool,
) -> String {
    let config = build_config(
        dialect,
        format,
        stress,
        include_stress,
        apply_sandhi,
        region,
        silesian_raising,
    );
    phonemize_with_config(text, &config)
}

#[pyfunction]
#[pyo3(signature = (texts, dialect = "warsaw", format = "ipa", stress = "prescriptive", include_stress = true, apply_sandhi = true, region = "katowice", silesian_raising = true))]
#[allow(clippy::too_many_arguments)]
fn phonemize_batch(
    py: Python<'_>,
    texts: Vec<String>,
    dialect: &str,
    format: &str,
    stress: &str,
    include_stress: bool,
    apply_sandhi: bool,
    region: &str,
    silesian_raising: bool,
) -> Vec<String> {
    let config = build_config(
        dialect,
        format,
        stress,
        include_stress,
        apply_sandhi,
        region,
        silesian_raising,
    );
    // Release GIL for multi-threaded rayon parallelism across all CPU cores
    py.allow_threads(|| {
        texts
            .par_iter()
            .map(|t| phonemize_with_config(t, &config))
            .collect()
    })
}

#[pyfunction]
#[pyo3(signature = (text, lang = "pl"))]
fn phonemize_lang(text: &str, lang: &str) -> String {
    tbenni_core::phonemize_lang(text, lang)
}

#[pyfunction]
#[pyo3(signature = (texts, lang = "pl"))]
fn phonemize_lang_batch(py: Python<'_>, texts: Vec<String>, lang: &str) -> Vec<String> {
    py.allow_threads(|| {
        texts
            .par_iter()
            .map(|t| tbenni_core::phonemize_lang(t, lang))
            .collect()
    })
}

/// Single-threaded batch phonemization (used for fair per-core throughput measurement).
/// `stage` selects the engine layers for the distilled languages: "base", "cart" or "full".
#[pyfunction]
#[pyo3(signature = (texts, lang = "pl", stage = "full"))]
fn phonemize_lang_seq(
    py: Python<'_>,
    texts: Vec<String>,
    lang: &str,
    stage: &str,
) -> PyResult<Vec<String>> {
    let stage = match Stage::parse(stage) {
        Some(s) => s,
        None => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "unknown stage: {stage}"
            )))
        }
    };
    let results = py.allow_threads(|| {
        texts
            .iter()
            .map(|t| tbenni_core::phonemize_lang_stage(t, lang, stage))
            .collect()
    });
    Ok(results)
}

/// Base-mapper segmentation `[(grapheme span, base phones), ...]` for a distilled language.
#[pyfunction]
fn base_segments(word: &str, lang: &str) -> PyResult<Vec<(String, String)>> {
    match tbenni_core::base_segments(word, lang) {
        Some(s) => Ok(s),
        None => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "no distilled engine for language: {lang}"
        ))),
    }
}

#[pymodule]
fn _tbenni(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(phonemize, m)?)?;
    m.add_function(wrap_pyfunction!(phonemize_batch, m)?)?;
    m.add_function(wrap_pyfunction!(phonemize_lang, m)?)?;
    m.add_function(wrap_pyfunction!(phonemize_lang_batch, m)?)?;
    m.add_function(wrap_pyfunction!(phonemize_lang_seq, m)?)?;
    m.add_function(wrap_pyfunction!(base_segments, m)?)?;
    Ok(())
}
