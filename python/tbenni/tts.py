"""TBenni TTS: End-to-End Neural Text-to-Speech Engine powered by TBenni G2P & Piper VITS.
"""

import os
import re
import time
import shutil
import tempfile
import subprocess
import urllib.request
from dataclasses import dataclass
from typing import Optional, List, Union, Tuple, Dict
from pathlib import Path

try:
    import numpy as np
    import soundfile as sf
    import piper
except ImportError as exc:  # pragma: no cover - depends on optional extras
    raise ImportError(
        "TBenni TTS requires optional dependencies. "
        "Install them with: pip install 'tbenni[tts]'"
    ) from exc

import tbenni

DEFAULT_VOICE = "gosia"

VOICE_CATALOG: Dict[str, Dict[str, str]] = {
    "tbenni_vits": {
        "lang": "pl",
        "gender": "female",
        "name": "model_tbenni",
        "sample_rate": 22050,
        "local_path": str(Path(__file__).resolve().parent.parent.parent / "models" / "tts_trained" / "model_tbenni.onnx"),
    },
    "espeak_vits": {
        "lang": "pl",
        "gender": "female",
        "name": "model_espeak",
        "sample_rate": 22050,
        "local_path": str(Path(__file__).resolve().parent.parent.parent / "models" / "tts_trained" / "model_espeak.onnx"),
    },
    "bass": {
        "lang": "pl",
        "gender": "male",
        "name": "pl_PL-bass-high",
        "sample_rate": 22050,
        "onnx_url": "https://huggingface.co/rhasspy/piper-voices/resolve/main/pl/pl_PL/bass/high/pl_PL-bass-high.onnx",
        "json_url": "https://huggingface.co/rhasspy/piper-voices/resolve/main/pl/pl_PL/bass/high/pl_PL-bass-high.onnx.json",
    },
    "mc_speech": {
        "lang": "pl",
        "gender": "female",
        "name": "pl_PL-mc_speech-medium",
        "sample_rate": 22050,
        "onnx_url": "https://huggingface.co/rhasspy/piper-voices/resolve/main/pl/pl_PL/mc_speech/medium/pl_PL-mc_speech-medium.onnx",
        "json_url": "https://huggingface.co/rhasspy/piper-voices/resolve/main/pl/pl_PL/mc_speech/medium/pl_PL-mc_speech-medium.onnx.json",
    },
    "gosia": {
        "lang": "pl",
        "gender": "female",
        "name": "pl_PL-gosia-medium",
        "sample_rate": 22050,
        "onnx_url": "https://huggingface.co/rhasspy/piper-voices/resolve/main/pl/pl_PL/gosia/medium/pl_PL-gosia-medium.onnx",
        "json_url": "https://huggingface.co/rhasspy/piper-voices/resolve/main/pl/pl_PL/gosia/medium/pl_PL-gosia-medium.onnx.json",
    },
    "darkman": {
        "lang": "pl",
        "gender": "male",
        "name": "pl_PL-darkman-medium",
        "sample_rate": 22050,
        "onnx_url": "https://huggingface.co/rhasspy/piper-voices/resolve/main/pl/pl_PL/darkman/medium/pl_PL-darkman-medium.onnx",
        "json_url": "https://huggingface.co/rhasspy/piper-voices/resolve/main/pl/pl_PL/darkman/medium/pl_PL-darkman-medium.onnx.json",
    },
    "jirka": {
        "lang": "cs",
        "gender": "male",
        "name": "cs_CZ-jirka-medium",
        "sample_rate": 22050,
        "onnx_url": "https://huggingface.co/rhasspy/piper-voices/resolve/main/cs/cs_CZ/jirka/medium/cs_CZ-jirka-medium.onnx",
        "json_url": "https://huggingface.co/rhasspy/piper-voices/resolve/main/cs/cs_CZ/jirka/medium/cs_CZ-jirka-medium.onnx.json",
    },
}


@dataclass
class TTSResult:
    """Result of TTS synthesis."""
    text: str
    audio: np.ndarray
    sample_rate: int
    duration_s: float
    phonemes: str
    phonemize_time_ms: float
    synth_time_ms: float
    total_time_ms: float
    rtf: float  # Real-Time Factor (total_time_s / duration_s)

    def save(self, filepath: Union[str, Path]) -> str:
        """Save synthesized audio to a WAV file."""
        path = str(filepath)
        os.makedirs(os.path.dirname(os.path.abspath(path)), exist_ok=True)
        sf.write(path, self.audio, self.sample_rate)
        return path

    def play(self) -> None:
        """Play audio using system player (paplay, aplay, mpv, or ffplay)."""
        player = None
        for p in ["paplay", "aplay", "mpv", "ffplay"]:
            if shutil.which(p):
                player = p
                break
        if not player:
            raise RuntimeError("No system audio player found (install paplay, aplay, mpv, or ffplay).")

        with tempfile.NamedTemporaryFile(suffix=".wav", delete=False) as f:
            tmp_path = f.name
        try:
            self.save(tmp_path)
            cmd = [player, tmp_path]
            if player == "ffplay":
                cmd = ["ffplay", "-nodisp", "-autoexit", tmp_path]
            elif player == "mpv":
                cmd = ["mpv", "--no-video", tmp_path]
            subprocess.run(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=True)
        finally:
            if os.path.exists(tmp_path):
                os.remove(tmp_path)


class TTSEngine:
    """TBenni Text-to-Speech Engine paired with Piper VITS acoustic models."""

    def __init__(
        self,
        voice: str = DEFAULT_VOICE,
        models_dir: Optional[Union[str, Path]] = None,
    ):
        self.voice_key = voice.lower()
        if models_dir:
            self.models_dir = Path(models_dir)
        else:
            # Default to repo models/tts or ~/.cache/tbenni/tts
            repo_models = Path(__file__).resolve().parent.parent.parent / "models" / "tts"
            if repo_models.exists():
                self.models_dir = repo_models
            else:
                self.models_dir = Path.home() / ".cache" / "tbenni" / "tts"

        self.models_dir.mkdir(parents=True, exist_ok=True)
        self.model_path, self.config_path = self._resolve_voice(self.voice_key)
        self.voice = piper.PiperVoice.load(str(self.model_path), str(self.config_path))
        self.sample_rate = self.voice.config.sample_rate

    def _resolve_voice(self, voice_key: str) -> Tuple[Path, Path]:
        """Resolve model paths, auto-downloading if missing."""
        if voice_key in VOICE_CATALOG:
            meta = VOICE_CATALOG[voice_key]
            if "local_path" in meta and Path(meta["local_path"]).exists():
                model_file = Path(meta["local_path"])
                config_file = model_file.with_suffix(".onnx.json")
                return model_file, config_file

            model_file = self.models_dir / f"{meta['name']}.onnx"
            config_file = self.models_dir / f"{meta['name']}.onnx.json"

            if not model_file.exists():
                print(f"Downloading TTS model '{meta['name']}'...")
                urllib.request.urlretrieve(meta["onnx_url"], model_file)
            if not config_file.exists():
                print(f"Downloading TTS config '{meta['name']}'...")
                urllib.request.urlretrieve(meta["json_url"], config_file)

            return model_file, config_file
        elif os.path.exists(voice_key):
            p = Path(voice_key)
            if p.suffix == ".onnx":
                cfg = p.with_suffix(".onnx.json")
                if not cfg.exists():
                    cfg = Path(str(p) + ".json")
                return p, cfg
            raise ValueError(f"Direct voice path must be an .onnx file: {voice_key}")
        else:
            available = list(VOICE_CATALOG.keys())
            raise ValueError(f"Unknown voice '{voice_key}'. Available catalog: {available}")

    def _phonemize_text(
        self,
        text: str,
        lang: str = "pl",
        soft_nasal: bool = True,
        remove_stress: bool = False,
    ) -> str:
        """Phonemize text using TBenni Native G2P engine and adapt prosody for VITS."""
        if lang == "pl":
            p_str = tbenni.phonemize(text, format="piper")
            if remove_stress:
                p_str = p_str.replace("ˈ", "").replace("ˌ", "")
            else:
                # Adapt stress placement to vowel nucleus and drop clitic stress
                VOWELS = set("aɛiɨɔuœæɐɑɒɘəɜɞɤɯɵ")
                CLITICS = {"pʒɨ", "pʃɨ", "f", "v", "z", "s", "na", "dɔ", "za", "u", "ɔ", "a", "i", "pɔd", "nad", "ɔd", "pʃɛz", "bɛz"}
                words = p_str.split(" ")
                adapted_words = []
                for w in words:
                    clean = re.sub(r"[.,!?;:\"\'ˈˌ]", "", w)
                    if clean in CLITICS:
                        w = w.replace("ˈ", "").replace("ˌ", "")
                    elif "ˈ" in w:
                        w_clean = w.replace("ˈ", "")
                        chars = list(w_clean)
                        v_idx = [i for i, c in enumerate(chars) if c in VOWELS]
                        if v_idx:
                            target_v = v_idx[-2] if len(v_idx) >= 2 else v_idx[0]
                            chars.insert(target_v, "ˈ")
                            w = "".join(chars)
                    adapted_words.append(w)
                p_str = " ".join(adapted_words)

            if soft_nasal:
                # Before affricates, smooth cluster to avoid unnatural double-nasal stutter (e.g. marznąc)
                p_str = p_str.replace("ɔnts", "ɔ̃ts").replace("ɛnts", "ɛ̃ts")
            return p_str
        elif lang == "cs":
            raw = tbenni.phonemize_lang(text, "cs")
            s = raw.replace("tʂ", "tʃ").replace("dʐ", "dʒ")
            s = s.replace("r̝", "r\u031d")
            return s
        else:
            return tbenni.phonemize_lang(text, lang)

    def synthesize(
        self,
        text: str,
        speed: float = 1.0,
        noise_scale: float = 0.667,
        noise_w: float = 0.8,
        speaker_id: Optional[int] = None,
        pause_duration_s: float = 0.25,
        soft_nasal: bool = True,
        remove_stress: bool = False,
        output_file: Optional[Union[str, Path]] = None,
    ) -> TTSResult:
        """Synthesize input text into high-fidelity speech.

        Args:
            text: Input string (sentence or paragraph).
            speed: Speech rate multiplier (1.0 = normal, 0.85 = faster, 1.15 = slower).
            noise_scale: Phoneme acoustic variability (0.667 default).
            noise_w: Phoneme duration variability (0.8 default).
            speaker_id: Speaker ID for multi-speaker models.
            pause_duration_s: Silence duration inserted between sentences in seconds.
            output_file: Optional path to save WAV output.

        Returns:
            TTSResult containing the synthesized audio and execution telemetry.
        """
        t_start = time.perf_counter()

        # Split into sentences for natural prosody and chunked synthesis
        sentences = [s.strip() for s in re.split(r"(?<=[.!?])\s+", text.strip()) if s.strip()]
        if not sentences:
            sentences = [text.strip()]

        lang = VOICE_CATALOG.get(self.voice_key, {}).get("lang", "pl")

        syn_config = piper.config.SynthesisConfig(
            speaker_id=speaker_id,
            length_scale=speed,
            noise_scale=noise_scale,
            noise_w_scale=noise_w,
        )

        all_phoneme_strs = []
        audio_chunks = []
        total_p_time = 0.0
        total_s_time = 0.0

        pause_samples = int(self.sample_rate * pause_duration_s)
        pause = np.zeros(pause_samples, dtype=np.float32) if pause_samples > 0 else None

        for idx, sentence in enumerate(sentences):
            # 1. TBenni Front-End Phonemization
            tp0 = time.perf_counter()
            p_str = self._phonemize_text(
                sentence, lang=lang, soft_nasal=soft_nasal, remove_stress=remove_stress
            )
            tp1 = time.perf_counter()
            total_p_time += (tp1 - tp0) * 1000.0
            all_phoneme_strs.append(p_str)

            # 2. Map phonemes to token IDs
            symbols = [c for c in p_str if c in self.voice.config.phoneme_id_map]
            ids = self.voice.phonemes_to_ids(symbols)

            # 3. Neural Acoustic Synthesis (VITS)
            ts0 = time.perf_counter()
            chunk = self.voice.phoneme_ids_to_audio(ids, syn_config=syn_config)
            ts1 = time.perf_counter()
            total_s_time += (ts1 - ts0) * 1000.0

            audio_chunks.append(chunk)
            if pause is not None and idx < len(sentences) - 1:
                audio_chunks.append(pause)

        full_audio = np.concatenate(audio_chunks) if len(audio_chunks) > 1 else audio_chunks[0]
        duration_s = len(full_audio) / float(self.sample_rate)
        t_end = time.perf_counter()
        total_time_ms = (t_end - t_start) * 1000.0
        rtf = (total_time_ms / 1000.0) / max(1e-4, duration_s)

        res = TTSResult(
            text=text,
            audio=full_audio,
            sample_rate=self.sample_rate,
            duration_s=duration_s,
            phonemes=" ".join(all_phoneme_strs),
            phonemize_time_ms=total_p_time,
            synth_time_ms=total_s_time,
            total_time_ms=total_time_ms,
            rtf=rtf,
        )

        if output_file:
            res.save(output_file)

        return res

    def speak(self, text: str, **kwargs) -> TTSResult:
        """Synthesize text and play it immediately."""
        res = self.synthesize(text, **kwargs)
        res.play()
        return res

    def compare_with_espeak(
        self,
        text: str,
        output_prefix: Optional[str] = None,
    ) -> Dict[str, Union[TTSResult, float, str]]:
        """Run side-by-side synthesis comparing TBenni vs eSpeak-NG front-end."""
        # 1. TBenni
        tbenni_res = self.synthesize(text)
        if output_prefix:
            tbenni_res.save(f"{output_prefix}_tbenni.wav")

        # 2. eSpeak
        t0 = time.perf_counter()
        espeak_sentences = self.voice.phonemize(text)
        t1 = time.perf_counter()
        espeak_p_time = (t1 - t0) * 1000.0

        audio_chunks = []
        espeak_phonemes = []
        t_synth0 = time.perf_counter()
        for sent in espeak_sentences:
            espeak_phonemes.append("".join(sent))
            ids = self.voice.phonemes_to_ids(sent)
            chunk = self.voice.phoneme_ids_to_audio(ids)
            audio_chunks.append(chunk)
        t_synth1 = time.perf_counter()

        espeak_audio = np.concatenate(audio_chunks) if len(audio_chunks) > 1 else audio_chunks[0]
        espeak_duration = len(espeak_audio) / float(self.sample_rate)
        espeak_total_time = (time.perf_counter() - t0) * 1000.0

        espeak_res = TTSResult(
            text=text,
            audio=espeak_audio,
            sample_rate=self.sample_rate,
            duration_s=espeak_duration,
            phonemes=" ".join(espeak_phonemes),
            phonemize_time_ms=espeak_p_time,
            synth_time_ms=(t_synth1 - t_synth0) * 1000.0,
            total_time_ms=espeak_total_time,
            rtf=(espeak_total_time / 1000.0) / max(1e-4, espeak_duration),
        )

        if output_prefix:
            espeak_res.save(f"{output_prefix}_espeak.wav")

        speedup = espeak_p_time / max(1e-4, tbenni_res.phonemize_time_ms)

        return {
            "tbenni": tbenni_res,
            "espeak": espeak_res,
            "speedup": speedup,
        }


if __name__ == "__main__":
    from tbenni.tts_cli import main
    main()
