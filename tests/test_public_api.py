"""Smoke tests for the installed distribution's public API."""

from importlib.metadata import version

import tbenni


def test_version_matches_installed_metadata():
    assert tbenni.__version__ == version("tbenni")


def test_public_names_are_exported():
    for name in (
        "phonemize",
        "phonemize_batch",
        "phonemize_lang",
        "phonemize_lang_batch",
        "phonemize_lang_seq",
        "base_segments",
    ):
        assert name in tbenni.__all__
        assert callable(getattr(tbenni, name))


def test_polish_batch_api():
    assert tbenni.phonemize_batch(["kot", "dom"], include_stress=False) == ["kɔt", "dɔm"]


def test_multilingual_api():
    assert isinstance(tbenni.phonemize_lang("čistý", "cs"), str)
    assert len(tbenni.phonemize_lang_batch(["čistý", "strom"], "cs")) == 2
