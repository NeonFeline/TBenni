"""TBenni: High-Performance Polish Phonemizer

Named in honor of Tytus Benni (1877-1935), the father of Polish experimental
phonetics.  The package exposes a fast Rust G2P engine for Polish (including
the Warsaw, Krakow/Poznan and Silesian dialects) plus distilled phonemizers
for Czech, Croatian, Bulgarian, Ukrainian and Russian.

Example:
    >>> import tbenni
    >>> tbenni.phonemize("czysta", include_stress=False)
    't͡ʂɨsta'

"""

from importlib.metadata import PackageNotFoundError, version as _version
from typing import List, Optional

from ._tbenni import base_segments, phonemize_lang, phonemize_lang_batch, phonemize_lang_seq
from ._tbenni import phonemize as _native_phonemize
from ._tbenni import phonemize_batch as _native_phonemize_batch

try:
    __version__ = _version("tbenni")
except PackageNotFoundError:  # running straight from a source checkout
    __version__ = "0.1.0"

__all__ = [
    "phonemize",
    "phonemize_batch",
    "phonemize_lang",
    "phonemize_lang_batch",
    "phonemize_lang_seq",
    "base_segments",
    "__version__",
    "TTSEngine",
    "TTSResult",
    "speak",
    "synthesize",
]

# Lazy-loaded default TTS engine instance
_default_engine = None


def _get_default_engine():
    global _default_engine
    if _default_engine is None:
        from .tts import TTSEngine
        _default_engine = TTSEngine()
    return _default_engine


def synthesize(text: str, voice: str = "gosia", speed: float = 1.0, output_file: Optional[str] = None):
    """Synthesize text into speech using TBenni TTS.

    Args:
        text: Text to synthesize.
        voice: Voice name ('gosia', 'darkman', 'jirka').
        speed: Speech rate multiplier.
        output_file: Optional file path to save audio.
    """
    from .tts import TTSEngine
    if voice == "gosia":
        engine = _get_default_engine()
    else:
        engine = TTSEngine(voice=voice)
    return engine.synthesize(text, speed=speed, output_file=output_file)


def speak(text: str, voice: str = "gosia", speed: float = 1.0):
    """Synthesize text and play it immediately through system audio."""
    res = synthesize(text, voice=voice, speed=speed)
    res.play()
    return res


def __getattr__(name):
    if name in ("TTSEngine", "TTSResult"):
        from . import tts
        return getattr(tts, name)
    raise AttributeError(f"module '{__name__}' has no attribute '{name}'")


def phonemize(
    text: str,
    lang: str = "pl",
    dialect: str = "warsaw",
    format: str = "ipa",
    stress: str = "prescriptive",
    include_stress: bool = True,
    apply_sandhi: bool = True,
    stage: str = "full",
    region: str = "katowice",
    silesian_raising: bool = True,
) -> str:
    """Phonemize text in Polish or other supported languages.

    Args:
        text: Input text string.
        lang: Language code ("pl", "cs", "hr", "bg", "uk", "ru", "de"). Default is "pl".
        dialect: Polish pronunciation dialect ("warsaw", "krakow", or "slask").
        format: Polish output format ("ipa", "ipa_broad", "sampa", "benni", "piper").
        stress: Polish stress assignment mode ("prescriptive" or "colloquial").
        include_stress: Whether to output stress diacritics.
        apply_sandhi: Whether to apply inter-word voicing assimilation.
        stage: Engine stage for distilled languages ("base", "cart", or "full").
        region: Silesian sub-region ("katowice", "opole", or "cieszyn"),
            controlling the regional reflex of word-final -ę (Silesian dialect only).
        silesian_raising: Whether to raise Silesian decomposed nasal nuclei
            (ą -> [o] in "idōm", "piōntek"; False keeps [ɔ]).

    Returns:
        Phonemized string.
    """
    if lang.lower() in ("pl", "polish"):
        return _native_phonemize(
            text=text,
            dialect=dialect,
            format=format,
            stress=stress,
            include_stress=include_stress,
            apply_sandhi=apply_sandhi,
            region=region,
            silesian_raising=silesian_raising,
        )
    return phonemize_lang_seq([text], lang=lang, stage=stage)[0]


def phonemize_batch(
    texts: List[str],
    lang: str = "pl",
    dialect: str = "warsaw",
    format: str = "ipa",
    stress: str = "prescriptive",
    include_stress: bool = True,
    apply_sandhi: bool = True,
    stage: str = "full",
    region: str = "katowice",
    silesian_raising: bool = True,
) -> List[str]:
    """Phonemize a batch of text strings concurrently across all CPU cores (releases GIL).

    Args:
        texts: List of input text strings.
        lang: Language code ("pl", "cs", "hr", "bg", "uk", "ru", "de"). Default is "pl".
        dialect: Polish pronunciation dialect ("warsaw", "krakow", or "slask").
        format: Polish output format ("ipa", "ipa_broad", "sampa", "benni", "piper").
        stress: Polish stress assignment mode ("prescriptive" or "colloquial").
        include_stress: Whether to output stress diacritics.
        apply_sandhi: Whether to apply inter-word voicing assimilation.
        stage: Engine stage for distilled languages ("base", "cart", or "full").
        region: Silesian sub-region ("katowice", "opole", or "cieszyn"),
            controlling the regional reflex of word-final -ę (Silesian dialect only).
        silesian_raising: Whether to raise Silesian decomposed nasal nuclei
            (ą -> [o] in "idōm", "piōntek"; False keeps [ɔ]).

    Returns:
        List of phonemized strings.
    """
    if lang.lower() in ("pl", "polish"):
        return _native_phonemize_batch(
            texts=texts,
            dialect=dialect,
            format=format,
            stress=stress,
            include_stress=include_stress,
            apply_sandhi=apply_sandhi,
            region=region,
            silesian_raising=silesian_raising,
        )
    return phonemize_lang_batch(texts, lang=lang)
