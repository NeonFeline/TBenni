# TBenni: High-Performance Polish Phonemizer

*Named in honor of Tytus Benni (1877-1935), the father of Polish experimental phonetics.*

TBenni is a blazingly fast, linguistically rigorous Polish Grapheme-to-Phoneme (G2P)
conversion engine and phonemizer written in Rust with Python bindings.

## Features
- **Dialect & Sandhi Support**:
  - `Warsaw` (Standard Central-Eastern Polish: inter-word devoicing).
  - `KrakowPoznan` (Standard Southern/Western Polish: inter-word voicing).
  - `Slask` (Silesian / Śląsk: decomposed nasal vowels, labialization, vowel raising, inter-word voicing).
- **Phonetic Output Formats**:
  - `IpaNarrow`: Allophonic narrow IPA transcription (e.g. `[t͡ʂɨsta]`, `[vɔw̃s]`, `[dɔmp]`).
  - `IpaBroad`: Broad phonemic IPA transcription (e.g. `/tʂɨsta/`, `/vɔws/`, `/dɔmp/`).
  - `Sampa` / `XSampa`: 7-bit ASCII representation.
  - `Benni` (Slavic Phonetic Alphabet - AS): Single-glyph affricates (`č, ǯ, ć, ʒ́, c, ʒ`).
  - `Piper`: Decomposed token map directly compatible with Piper TTS ONNX models.
- **Stress Modes**:
  - `Prescriptive`: Standard antepenultimate stress on Latin borrowings (*fizyka*, *matematyka*) and past-tense verbs (*byliśmy*, *zrobilibyśmy*).
  - `Colloquial`: Regularized penultimate stress.

## Building

Requires a Rust toolchain and, for the Python bindings, [maturin](https://maturin.rs).

```sh
# Rust library and CLI
cargo build --release -p tbenni-cli        # binary at target/release/tbenni
cargo test -p tbenni-core                  # engine test suite

# Python extension module (inside a virtualenv)
pip install maturin
maturin develop --release
```

Single-thread throughput benchmark (takes a word list, one word per line):

```sh
cargo run --release -p tbenni-core --example bench_pl -- words.txt
```

## Usage

### CLI

```sh
$ tbenni --no-stress "odznaczyć"
ɔdznat͡ʂɨt͡ɕ

$ echo "czysta trzysta" | tbenni --no-stress
t͡ʂɨsta tʂɨsta
```

Options: `--dialect warsaw|krakow|slask` (default `krakow`), `--format ipa-narrow|ipa-broad|sampa|benni|piper`,
`--stress prescriptive|colloquial`, `--no-stress`, `--no-sandhi`.

### Python

```python
import tbenni

tbenni.phonemize("odznaczyć", include_stress=False)   # 'ɔdznat͡ʂɨt͡ɕ'
tbenni.phonemize("brat ojca", dialect="krakow",
                 include_stress=False)                # 'brad ɔjt͡sa'
tbenni.phonemize_batch(["kot", "dom"])                # multi-core batch
```

`phonemize(text, dialect="warsaw"|"krakow"|"slask",
format="ipa"|"ipa_broad"|"sampa"|"benni"|"piper",
stress="prescriptive"|"colloquial", include_stress=..., apply_sandhi=...)`

### Rust

```rust
use tbenni_core::{phonemize_with_config, Config};

let ipa = phonemize_with_config(
    "brat ojca",
    &Config { include_stress: false, ..Default::default() },
);
assert_eq!(ipa, "brad ɔjt͡sa"); // KrakowPoznan is the default dialect
```

## Tests

```sh
cargo test -p tbenni-core
pytest tests/test_tbenni.py
```

## License

MIT OR Apache-2.0. See `LICENSE-MIT` and `LICENSE-APACHE`.
