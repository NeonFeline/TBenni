# TBenni: High-Performance Polish Phonemizer

*Named in honor of Tytus Benni (1877–1935), the father of Polish experimental phonetics.*

[![CI](https://github.com/NeonFeline/TBenni/actions/workflows/ci.yml/badge.svg)](https://github.com/NeonFeline/TBenni/actions/workflows/ci.yml)
[![PyPI](https://img.shields.io/pypi/v/tbenni.svg)](https://pypi.org/project/tbenni/)
[![Python versions](https://img.shields.io/pypi/pyversions/tbenni.svg)](https://pypi.org/project/tbenni/)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

TBenni is a blazingly fast, linguistically rigorous Polish Grapheme-to-Phoneme (G2P)
conversion engine and phonemizer written in Rust with Python bindings.

## Installation

```sh
pip install tbenni
```

The core package has no Python dependencies. Optional text-to-speech support
(Piper VITS voices, NumPy, soundfile) is available as an extra:

```sh
pip install "tbenni[tts]"
```

## Quick start

```python
import tbenni

# Narrow IPA with the default (Warsaw) dialect
tbenni.phonemize("czysta", include_stress=False)
# 't͡ʂɨsta'

tbenni.phonemize("Warszawa")
# 'varˈʂava'

# Batch conversion releases the GIL and uses all CPU cores
tbenni.phonemize_batch(["kot", "dom"], include_stress=False)
# ['kɔt', 'dɔm']

# Silesian dialect and inter-word sandhi
tbenni.phonemize("idą", dialect="slask", include_stress=False)
# 'idom'
```

The multilingual engine is available through `phonemize_lang` and
`phonemize_lang_batch`:

```python
tbenni.phonemize_lang("čistý", "cs")
# 'tʂɪstiː'
```

### Text-to-speech

```python
import tbenni

audio = tbenni.synthesize("Witaj w świecie TBenni!", voice="gosia")
audio.save("witaj.wav")
audio.play()
```

Or from the command line:

```sh
tbenni-tts "Witaj w świecie TBenni!" --output witaj.wav --play
tbenni-tts --list-voices
tbenni-web            # interactive browser studio at http://127.0.0.1:8080
```

## Features

- **Dialect & Sandhi Support**:
  - `Warsaw` (Standard Central-Eastern Polish: inter-word devoicing).
  - `KrakowPoznan` (Standard Southern/Western Polish: inter-word voicing).
  - `Slask` (Silesian / Ślōnskŏ gŏdka: initial o- labialization `[wɔknɔ]`, é → `[ɨ]`, inter-word voicing, decomposed nasals *nosówki rozłożone* `idą → [idom]`, `piątek → [pʲontɛk]`, with regional word-final `-ę` reflexes via `region`: Katowice `[a]`, Opole `[ã]`, Cieszyn `[e]`).
- **Phonetic Output Formats**:
  - `IpaNarrow`: Allophonic narrow IPA transcription (e.g. `[t͡ʂɨsta]`, `[vɔw̃s]`, `[dɔmp]`).
  - `IpaBroad`: Broad phonemic IPA transcription (e.g. `/tʂɨsta/`, `/vɔws/`, `/dɔmp/`).
  - `Sampa` / `XSampa`: 7-bit ASCII representation.
  - `Benni` (Slavic Phonetic Alphabet - AS): Single-glyph affricates (`č, ǯ, ć, ʒ́, c, ʒ`).
  - `Piper`: Decomposed token map compatible with Piper TTS ONNX models (validated against the pl_PL phoneme_id_map).
- **Stress Modes**:
  - `Prescriptive`: Standard antepenultimate stress on Latin borrowings (*fizyka*, *matematyka*) and past-tense verbs (*byliśmy*, *zrobilibyśmy*).
  - `Colloquial`: Regularized penultimate stress.
- **Batch API**: `phonemize_batch` releases the GIL and parallelizes across all CPU cores.
- **Multilingual engines**: Distilled Czech, Croatian, Bulgarian, Ukrainian and Russian G2P.

## API

| Function | Description |
| --- | --- |
| `phonemize(text, ...)` | Phonemize one string (Polish or a distilled language). |
| `phonemize_batch(texts, ...)` | Parallel phonemization of many strings. |
| `phonemize_lang(text, lang, ...)` | Phonemize a string with a distilled language engine. |
| `phonemize_lang_batch(texts, lang, ...)` | Parallel multilingual batch conversion. |
| `phonemize_lang_seq(texts, lang, stage=...)` | Single-threaded batch conversion (`base`, `cart`, `full`). |
| `base_segments(word, lang)` | Base-mapper segmentation for a distilled language. |

Main Polish options:

- `dialect`: `"warsaw"` (default), `"krakow"`, `"slask"`
- `format`: `"ipa"` (default), `"ipa_broad"`, `"sampa"`, `"benni"`, `"piper"`
- `stress`: `"prescriptive"` (default) or `"colloquial"`
- `include_stress`: output stress marks (default `True`)
- `apply_sandhi`: inter-word voicing assimilation (default `True`)
- `region`: Silesian `-ę` reflex: `"katowice"` (default), `"opole"`, `"cieszyn"`
- `silesian_raising`: raise Silesian nasal nuclei (default `True`)

## Build from source

Requires a Rust toolchain and [maturin](https://maturin.rs).

```sh
# Rust library and CLI
cargo build --release -p tbenni-cli        # binary at target/release/tbenni
cargo test --all                           # Rust test suite

# Python extension
pip install maturin
maturin develop --release                  # install into the active virtualenv
pytest                                     # Python test suite
maturin build --release --out dist         # build a wheel
```

## Multilingual G2P evaluation

Beyond the hand-written Polish engine, the repository contains a distillation pipeline and a
contamination-controlled benchmark for Czech, Croatian, Bulgarian, Ukrainian and Russian
(plus Polish as reference), described in `paper/main.tex`:

- `pipeline/build_test_sets.py` — draws dev/test splits from the MFA dictionaries, excluding
  every word in the seed lexicon, the CharsiuG2P training lexicon and the Polish development
  sets; `pipeline/normalize.py` — the frozen scoring normalizer.
- `pipeline/distill_engine.py` + `pipeline/codegen_rust.py` — train per-grapheme CART
  corrections from a teacher's pool annotations and compile them (plus the exception slice)
  into `crates/tbenni-core/src/langs/<lang>.rs`.
- `pipeline/blind_mfa_eval.py` — scores TBenni (base/CART/full stages), the neural teacher,
  CharsiuG2P, Epitran and eSpeak-NG with bootstrap CIs; `results/eval_test.json`.
- `pipeline/compare_extractors.py` + `pipeline/extractors.py` — rule-extraction comparison
  (segment CART, TBL, context table, letter CART, Phonetisaurus) under the CharsiuG2P teacher,
  the TBenni teacher, or no teacher (seed lexicon only).
- `scripts/generate_paper_tables.py`, `scripts/generate_extractor_table.py`,
  `scripts/analyze_engine_pool_overlap.py` — regenerate every number/table/figure in the paper
  from the results files.

Typical run: `python pipeline/compare_extractors.py --teacher own`, then
`python scripts/generate_extractor_table.py`.

## Releasing

Releases are built and published by GitHub Actions. Pushing a tag such as `v0.1.0`
builds wheels for Linux, macOS and Windows, creates a GitHub Release, and uploads the
artifacts to PyPI using trusted publishing. See
[`.github/workflows/release.yml`](.github/workflows/release.yml).

## License

Dual-licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
