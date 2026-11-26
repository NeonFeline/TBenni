"""Command-line interface for TBenni Text-to-Speech synthesis.
"""

import sys
import argparse
from pathlib import Path
from typing import Optional
import tbenni.tts as tts


def main():
    parser = argparse.ArgumentParser(
        prog="tbenni-tts",
        description="TBenni Neural Text-to-Speech Engine (TBenni G2P + Piper VITS)",
    )
    parser.add_argument("text", nargs="?", default=None, help="Text to synthesize")
    parser.add_argument("-v", "--voice", default="gosia", choices=["gosia", "darkman", "jirka"], help="Voice model (default: gosia)")
    parser.add_argument("-o", "--output", default=None, help="Path to save output WAV file (e.g. output.wav)")
    parser.add_argument("-p", "--play", action="store_true", help="Play synthesized audio immediately via system audio player")
    parser.add_argument("-s", "--speed", type=float, default=1.0, help="Speech rate (1.0 = normal, 0.85 = faster, 1.15 = slower)")
    parser.add_argument("-c", "--compare", action="store_true", help="Compare side-by-side with eSpeak-NG front-end")
    parser.add_argument("-i", "--interactive", action="store_true", help="Interactive REPL mode (read from stdin, speak immediately)")
    parser.add_argument("-w", "--web", action="store_true", help="Launch interactive Web GUI Studio in browser")
    parser.add_argument("--port", type=int, default=8080, help="Port for Web GUI Studio (default: 8080)")
    parser.add_argument("--list-voices", action="store_true", help="List available voice models")

    args = parser.parse_args()

    if args.list_voices:
        print("Available Voices in TBenni TTS Catalog:")
        for k, v in tts.VOICE_CATALOG.items():
            print(f"  • {k:8s} | Lang: {v['lang']} | Gender: {v['gender']:6s} | Model: {v['name']}")
        return

    if args.web:
        from tbenni.web_ui import run_studio
        run_studio(port=args.port)
        return

    engine = tts.TTSEngine(voice=args.voice)

    if args.interactive:
        print("=" * 60)
        print(f"TBenni TTS Interactive Shell [Voice: {args.voice}]")
        print("Type text and press Enter to synthesize and play.")
        print("Type 'exit' or Ctrl+C to quit.")
        print("=" * 60)
        idx = 1
        while True:
            try:
                line = input(f"\n[{idx}] ❯ ").strip()
                if not line:
                    continue
                if line.lower() in ("exit", "quit", "q"):
                    break

                res = engine.synthesize(line, speed=args.speed)
                out_path = f"synth_{idx:03d}.wav"
                res.save(out_path)
                print(f"  Phonemes: {res.phonemes}")
                print(f"  Speed:    G2P = {res.phonemize_time_ms:.3f} ms | VITS = {res.synth_time_ms:.1f} ms | RTF = {res.rtf:.3f}")
                print(f"  Saved:    {out_path}")
                try:
                    res.play()
                except Exception as e:
                    print(f"  Playback error: {e}")
                idx += 1
            except (KeyboardInterrupt, EOFError):
                print("\nExiting.")
                break
        return

    if not args.text:
        # Check if stdin has text piped
        if not sys.stdin.isatty():
            args.text = sys.stdin.read().strip()
        else:
            parser.print_help()
            return

    if args.compare:
        print(f"Running side-by-side comparison for: \"{args.text}\"\n")
        out_prefix = Path(args.output).stem if args.output else "comparison"
        comp = engine.compare_with_espeak(args.text, output_prefix=out_prefix)
        tb = comp["tbenni"]
        es = comp["espeak"]
        speedup = comp["speedup"]

        print(f"TBenni Native Front-End:")
        print(f"  Phonemes: {tb.phonemes}")
        print(f"  G2P Time: {tb.phonemize_time_ms:.3f} ms")
        print(f"  Audio:    {out_prefix}_tbenni.wav ({tb.duration_s:.2f} s)\n")

        print(f"eSpeak-NG Front-End:")
        print(f"  Phonemes: {es.phonemes}")
        print(f"  G2P Time: {es.phonemize_time_ms:.3f} ms")
        print(f"  Audio:    {out_prefix}_espeak.wav ({es.duration_s:.2f} s)\n")

        print(f"Speedup: TBenni G2P is {speedup:.1f}x FASTER than eSpeak-NG front-end.")

        if args.play:
            print("\nPlaying TBenni audio...")
            tb.play()
            print("Playing eSpeak audio...")
            es.play()
        return

    # Standard single synthesis
    out_file = args.output or "output.wav"
    res = engine.synthesize(args.text, speed=args.speed, output_file=out_file)
    print(f"Synthesized: \"{args.text}\"")
    print(f"  Phonemes:  {res.phonemes}")
    print(f"  Latency:   G2P = {res.phonemize_time_ms:.3f} ms | VITS = {res.synth_time_ms:.1f} ms | Total = {res.total_time_ms:.1f} ms")
    print(f"  RTF:       {res.rtf:.3f} ({1.0/max(1e-4, res.rtf):.1f}x real-time speed)")
    print(f"  Output:    {out_file} ({res.duration_s:.2f} s)")

    if args.play:
        print("  Playing audio...")
        res.play()


if __name__ == "__main__":
    main()
