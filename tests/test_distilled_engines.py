"""Invariants of the distilled engines (cs, hr, bg, uk, ru): base rules -> CART -> exceptions."""

import json
import os

import pytest
import tbenni

DISTILLED = ["bg", "cs", "hr", "ru", "uk"]
SAMPLES = {
    "cs": ["dětství", "chleba", "mě", "nic", "ďábel"],
    "hr": ["ljubav", "njega", "džep", "ćevapi", "čovjek"],
    "bg": ["ябълка", "юнак", "град", "щастие", "всъщност"],
    "uk": ["їжак", "ґанок", "щастя", "м'ясо", "будь"],
    "ru": ["ёлка", "объём", "сделать", "щука", "дождь"],
}


@pytest.mark.parametrize("lang", DISTILLED)
def test_segments_cover_word(lang):
    for word in SAMPLES[lang]:
        segs = tbenni.base_segments(word, lang)
        assert "".join(span for span, _ in segs) == word.lower()


@pytest.mark.parametrize("lang", DISTILLED)
def test_base_stage_is_concatenated_segments(lang):
    words = SAMPLES[lang]
    base = tbenni.phonemize_lang_seq(words, lang=lang, stage="base")
    assert base == ["".join(p for _, p in tbenni.base_segments(w, lang)) for w in words]


@pytest.mark.parametrize("lang", DISTILLED)
def test_full_is_cart_plus_exception_slice(lang):
    path = os.path.join("data", lang, f"distilled_rules_{lang}.json")
    if not os.path.exists(path):
        pytest.skip("no distilled rules for this language")
    with open(path, encoding="utf-8") as f:
        exceptions = {e["word"]: e["ipa"] for e in json.load(f)["exceptions"]}
    words = SAMPLES[lang] + sorted(exceptions)[:50]
    cart = tbenni.phonemize_lang_seq(words, lang=lang, stage="cart")
    full = tbenni.phonemize_lang_seq(words, lang=lang, stage="full")
    assert full == [exceptions.get(w, c) for w, c in zip(words, cart)]


def test_parallel_and_sequential_agree():
    words = SAMPLES["cs"] * 10
    assert tbenni.phonemize_lang_batch(words, lang="cs") == tbenni.phonemize_lang_seq(words, lang="cs")


def test_unknown_stage_raises():
    with pytest.raises(ValueError):
        tbenni.phonemize_lang_seq(["slovo"], lang="cs", stage="bogus")


def test_base_segments_rejects_non_distilled_language():
    with pytest.raises(ValueError):
        tbenni.base_segments("słowo", "pl")
