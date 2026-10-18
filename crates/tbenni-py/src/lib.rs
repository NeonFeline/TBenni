use pyo3::prelude::*;
use rayon::prelude::*;
use tbenni_core::{
    phonemize_with_config, Config, Dialect as CoreDialect, OutputFormat as CoreOutputFormat,
    StressMode as CoreStressMode,
};

fn build_config(
    dialect: &str,
    format: &str,
    stress: &str,
    include_stress: bool,
    apply_sandhi: bool,
) -> Config {
    let d = match dialect.to_lowercase().as_str() {
        "warsaw" | "warszawa" | "mazovia" => CoreDialect::Warsaw,
        "krakow" | "krakow_poznan" | "poznan" | "standard" => CoreDialect::KrakowPoznan,
        "slask" | "silesian" | "silesia" | "slonsk" => CoreDialect::Slask,
        _ => CoreDialect::KrakowPoznan,
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

    Config {
        dialect: d,
        format: f,
        stress_mode: s,
        include_stress,
        apply_sandhi,
        ..Default::default()
    }
}

#[pyfunction]
#[pyo3(signature = (text, dialect = "krakow", format = "ipa", stress = "prescriptive", include_stress = true, apply_sandhi = true))]
fn phonemize(
    text: &str,
    dialect: &str,
    format: &str,
    stress: &str,
    include_stress: bool,
    apply_sandhi: bool,
) -> PyResult<String> {
    let config = build_config(dialect, format, stress, include_stress, apply_sandhi);
    Ok(phonemize_with_config(text, &config))
}

#[pyfunction]
#[pyo3(signature = (texts, dialect = "krakow", format = "ipa", stress = "prescriptive", include_stress = true, apply_sandhi = true))]
fn phonemize_batch(
    py: Python<'_>,
    texts: Vec<String>,
    dialect: &str,
    format: &str,
    stress: &str,
    include_stress: bool,
    apply_sandhi: bool,
) -> PyResult<Vec<String>> {
    let config = build_config(dialect, format, stress, include_stress, apply_sandhi);
    // Release GIL for multi-threaded rayon parallelism across all CPU cores
    let results = py.allow_threads(|| {
        texts
            .par_iter()
            .map(|t| phonemize_with_config(t, &config))
            .collect()
    });
    Ok(results)
}

#[pymodule]
fn _tbenni(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(phonemize, m)?)?;
    m.add_function(wrap_pyfunction!(phonemize_batch, m)?)?;
    Ok(())
}
