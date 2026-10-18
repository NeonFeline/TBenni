"""TBenni: High-Performance Polish Phonemizer
Named in honor of Tytus Benni (1877–1935).
"""

from typing import List
from ._tbenni import phonemize as _native_phonemize
from ._tbenni import phonemize_batch as _native_phonemize_batch

__version__ = "0.1.0"
__all__ = [
    "phonemize",
    "phonemize_batch",
]


def phonemize(
    text: str,
    dialect: str = "krakow",
    format: str = "ipa",
    stress: str = "prescriptive",
    include_stress: bool = True,
    apply_sandhi: bool = True,
) -> str:
    """Phonemize Polish text.

    Args:
        text: Polish input text string.
        dialect: Pronunciation dialect ("warsaw", "krakow", or "slask").
        format: Output format ("ipa", "ipa_broad", "sampa", "benni", "piper").
        stress: Stress assignment mode ("prescriptive" or "colloquial").
        include_stress: Whether to output stress diacritics.
        apply_sandhi: Whether to apply inter-word voicing assimilation.

    Returns:
        Phonemized string.
    """
    return _native_phonemize(
        text=text,
        dialect=dialect,
        format=format,
        stress=stress,
        include_stress=include_stress,
        apply_sandhi=apply_sandhi,
    )

def phonemize_batch(
    texts: List[str],
    dialect: str = "krakow",
    format: str = "ipa",
    stress: str = "prescriptive",
    include_stress: bool = True,
    apply_sandhi: bool = True,
) -> List[str]:
    """Phonemize a batch of Polish text strings concurrently across all CPU cores (releases GIL).

    Args:
        texts: List of Polish input text strings.
        dialect: Pronunciation dialect ("warsaw", "krakow", or "slask").
        format: Output format ("ipa", "ipa_broad", "sampa", "benni", "piper").
        stress: Stress assignment mode ("prescriptive" or "colloquial").
        include_stress: Whether to output stress diacritics.
        apply_sandhi: Whether to apply inter-word voicing assimilation.

    Returns:
        List of phonemized strings.
    """
    return _native_phonemize_batch(
        texts=texts,
        dialect=dialect,
        format=format,
        stress=stress,
        include_stress=include_stress,
        apply_sandhi=apply_sandhi,
    )
