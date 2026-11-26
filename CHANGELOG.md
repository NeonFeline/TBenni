# Changelog

All notable changes to TBenni are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses
[Semantic Versioning](https://semver.org/).

## [0.1.0] - 2026-11-26

### Added

- First public Python release of the TBenni G2P engine.
- Polish phonemizer with Warsaw, Krakow/Poznan and Silesian dialects.
- Narrow/broad IPA, SAMPA, Benni (Slavic Phonetic Alphabet) and Piper output.
- Prescriptive and colloquial stress modes.
- Distilled Czech, Croatian, Bulgarian, Ukrainian and Russian engines.
- `phonemize`, `phonemize_batch`, `phonemize_lang`, `phonemize_lang_batch`,
  `phonemize_lang_seq` and `base_segments` APIs.
- Optional `tbenni[tts]` extra with Piper VITS synthesis and `tbenni-tts` /
  `tbenni-web` commands.
- GitHub Actions CI and tag-triggered wheel, sdist, GitHub Release and PyPI
  publishing workflows.
