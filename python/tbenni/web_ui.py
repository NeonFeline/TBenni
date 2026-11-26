"""Interactive Web GUI for TBenni Neural Speech Synthesis Studio.
Runs a lightweight local web server with zero external dependencies (pure Python http.server).
"""

import os
import io
import json
import base64
import urllib.parse
from http.server import HTTPServer, BaseHTTPRequestHandler
import soundfile as sf
import tbenni.tts as tts

HTML_TEMPLATE = """<!DOCTYPE html>
<html lang="pl">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>TBenni Neural Speech Studio</title>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
  <style>
    :root {
      --bg: #0b0f19;
      --card: #131b2e;
      --border: #1e293b;
      --text: #f8fafc;
      --muted: #94a3b8;
      --primary: #6366f1;
      --primary-hover: #4f46e5;
      --accent: #10b981;
      --warning: #f59e0b;
      --danger: #ef4444;
    }
    * { box-sizing: border-box; margin: 0; padding: 0; }
    body {
      background: var(--bg);
      color: var(--text);
      font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
      padding: 2rem 1rem;
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
    }
    .container {
      max-width: 900px;
      width: 100%;
    }
    header {
      text-align: center;
      margin-bottom: 2rem;
    }
    .badge {
      display: inline-block;
      padding: 0.25rem 0.75rem;
      font-size: 0.75rem;
      font-weight: 600;
      text-transform: uppercase;
      letter-spacing: 0.05em;
      border-radius: 9999px;
      background: rgba(99, 102, 241, 0.15);
      color: #818cf8;
      border: 1px solid rgba(99, 102, 241, 0.3);
      margin-bottom: 0.75rem;
    }
    h1 {
      font-size: 2.25rem;
      font-weight: 700;
      background: linear-gradient(135deg, #ffffff 0%, #cbd5e1 100%);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      margin-bottom: 0.5rem;
    }
    p.subtitle {
      color: var(--muted);
      font-size: 1rem;
    }
    .card {
      background: var(--card);
      border: 1px solid var(--border);
      border-radius: 1rem;
      padding: 1.75rem;
      box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.5);
      margin-bottom: 1.5rem;
    }
    label {
      display: block;
      font-size: 0.875rem;
      font-weight: 600;
      color: var(--muted);
      margin-bottom: 0.5rem;
      text-transform: uppercase;
      letter-spacing: 0.05em;
    }
    select, textarea, input[type="range"] {
      width: 100%;
      background: #0f172a;
      border: 1px solid var(--border);
      color: var(--text);
      border-radius: 0.5rem;
      padding: 0.75rem 1rem;
      font-family: inherit;
      font-size: 0.95rem;
      transition: border-color 0.2s;
    }
    select:focus, textarea:focus {
      outline: none;
      border-color: var(--primary);
    }
    textarea {
      resize: vertical;
      min-height: 110px;
      line-height: 1.5;
    }
    .grid {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 1.25rem;
      margin-top: 1.25rem;
    }
    @media (max-width: 640px) {
      .grid { grid-template-columns: 1fr; }
    }
    .slider-group {
      display: flex;
      flex-direction: column;
      gap: 0.35rem;
    }
    .slider-val {
      float: right;
      color: var(--primary);
      font-family: 'JetBrains Mono', monospace;
      font-size: 0.85rem;
    }
    .btn-group {
      display: flex;
      gap: 1rem;
      margin-top: 1.5rem;
    }
    button {
      flex: 1;
      padding: 0.85rem 1.5rem;
      font-weight: 600;
      font-size: 0.95rem;
      border-radius: 0.5rem;
      border: none;
      cursor: pointer;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      gap: 0.5rem;
      transition: all 0.2s;
    }
    .btn-primary {
      background: var(--primary);
      color: white;
    }
    .btn-primary:hover {
      background: var(--primary-hover);
      transform: translateY(-1px);
    }
    .btn-secondary {
      background: #1e293b;
      color: var(--text);
      border: 1px solid var(--border);
    }
    .btn-secondary:hover {
      background: #334155;
    }
    .results-grid {
      display: grid;
      grid-template-columns: 1fr;
      gap: 1.25rem;
      margin-top: 1.5rem;
    }
    .result-box {
      background: #0f172a;
      border: 1px solid var(--border);
      border-radius: 0.75rem;
      padding: 1.25rem;
    }
    .result-box.tbenni {
      border-left: 4px solid var(--accent);
    }
    .result-box.espeak {
      border-left: 4px solid var(--warning);
    }
    .box-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 0.75rem;
    }
    .box-title {
      font-size: 1.1rem;
      font-weight: 600;
      display: flex;
      align-items: center;
      gap: 0.5rem;
    }
    .telemetry {
      font-family: 'JetBrains Mono', monospace;
      font-size: 0.8rem;
      color: var(--muted);
      display: flex;
      gap: 1rem;
    }
    .telemetry span b {
      color: var(--text);
    }
    .phoneme-display {
      font-family: 'JetBrains Mono', monospace;
      background: #090d16;
      padding: 0.75rem 1rem;
      border-radius: 0.5rem;
      font-size: 0.9rem;
      color: #38bdf8;
      word-break: break-all;
      margin-bottom: 0.75rem;
      border: 1px solid rgba(255,255,255,0.05);
    }
    audio {
      width: 100%;
      height: 40px;
      border-radius: 0.5rem;
      outline: none;
    }
    .speedup-pill {
      background: rgba(16, 185, 129, 0.15);
      color: #34d399;
      border: 1px solid rgba(16, 185, 129, 0.3);
      padding: 0.2rem 0.6rem;
      border-radius: 9999px;
      font-size: 0.8rem;
      font-weight: 600;
    }
    .spinner {
      display: inline-block;
      width: 1rem;
      height: 1rem;
      border: 2px solid rgba(255,255,255,0.3);
      border-radius: 50%;
      border-top-color: white;
      animation: spin 0.8s linear infinite;
    }
    @keyframes spin { to { transform: rotate(360deg); } }
  </style>
</head>
<body>
  <div class="container">
    <header>
      <div class="badge">TBenni Neuro-Symbolic Speech Studio</div>
      <h1>TBenni Neural Text-to-Speech</h1>
      <p class="subtitle">Sub-microsecond G2P distillation paired with Piper VITS neural acoustic synthesis</p>
    </header>

    <div class="card">
      <div style="margin-bottom: 1.25rem;">
        <label for="presets">Przykłady / Testy fonetyczne:</label>
        <select id="presets" onchange="applyPreset()">
          <option value="">-- Wybierz test fonetyczny --</option>
          <option value="odznaczeniu">🎯 Słowo: 'odznaczeniu' (eSpeak: 'odź-na-cze-niu' vs TBenni: 'od-zna-cze-niu')</option>
          <option value="zmarzł">🎯 Słowo: 'zmarzł' (eSpeak obcina 'ł' -> 'zmarz' vs TBenni: 'zmarzł')</option>
          <option value="zmarzlak">🎯 Słowo: 'zmarzlak' (eSpeak: 'zmażlak' vs TBenni: 'zmarzlak')</option>
          <option value="podzamcza">🎯 Słowo: 'podzamcza' (eSpeak: 'podźamcza' vs TBenni: 'podzamcza')</option>
          <option value="marznąć">🎯 Słowo: 'marznąć' (eSpeak: 'marznonts' vs TBenni: 'marznõts')</option>
          <option value="Zmarzlak zmarzł, marznąc przy odznaczeniu podzamcza.">Zdanie: Granice morfemów (zmarzlak, odznaczeniu, podzamcze)</option>
          <option value="W Szczebrzeszynie chrząszcz brzmi w trzcinie, a trzmiel bzyczy w czynie.">Zdanie: Zbitki spółgłoskowe i asymilacje dźwięczności</option>
          <option value="Dąb rośnie przy kącie, a pięć pstrągów płynie w głąb rzeki.">Zdanie: Samogłoski nosowe (rozszczepienie przed zwartymi)</option>
          <option value="Czeszą się czyste czaple czyśćcem na czczo.">Zdanie: Szeregi syczące i ciszące (cz vs c)</option>
          <option value="Witaj! Nazywam się TBenni i potrafię syntetyzować mowę z ultra niską latencją.">Zdanie: Naturalne powitanie (mowa ciągła)</option>
          <option value="Dobrý den, vítám vás u české syntézy řeči.">Czeski (cs): Dobrý den, vítám vás</option>
        </select>
      </div>

      <div style="margin-bottom: 1.25rem;">
        <label for="text-input">Tekst do syntezy:</label>
        <textarea id="text-input" placeholder="Wpisz dowolny tekst po polsku lub czesku...">Zmarzlak zmarzł, marznąc przy odznaczeniu podzamcza.</textarea>
      </div>

      <div class="grid">
        <div>
          <label for="voice-select">Głos / Język:</label>
          <select id="voice-select">
            <option value="tbenni_vits" selected>⭐ TBenni-VITS (Trained on RTX 3090 with TBenni G2P)</option>
            <option value="espeak_vits">⚠️ eSpeak-VITS (Trained on RTX 3090 with eSpeak G2P)</option>
            <option value="bass">Bass HQ (Pretrained Polish Male HQ, 22.05 kHz)</option>
            <option value="mc_speech">MC Speech (Pretrained Polish Female Studio, 22.05 kHz)</option>
            <option value="gosia">Gosia (Pretrained Polish Female, 22.05 kHz)</option>
            <option value="darkman">Darkman (Pretrained Polish Male, 22.05 kHz)</option>
            <option value="jirka">Jirka (Czech Male, 22.05 kHz)</option>
          </select>
        </div>
        <div class="slider-group">
          <label for="speed-slider">Szybkość mowy (Speed): <span id="speed-val" class="slider-val">1.0x</span></label>
          <input type="range" id="speed-slider" min="0.5" max="1.5" step="0.05" value="1.0" oninput="updateSlider('speed-val', this.value + 'x')">
        </div>
      </div>

      <div class="btn-group">
        <button id="btn-synth" class="btn-primary" onclick="synthesize(false)">
          <span>⚡ Syntezuj TBenni</span>
        </button>
        <button id="btn-compare" class="btn-secondary" onclick="synthesize(true)">
          <span>⚔️ Porównaj A/B (TBenni vs eSpeak)</span>
        </button>
      </div>
    </div>

    <div id="results-area" class="results-grid" style="display:none;"></div>
  </div>

  <script>
    function updateSlider(id, val) {
      document.getElementById(id).textContent = val;
    }

    function applyPreset() {
      const select = document.getElementById('presets');
      if (select.value) {
        document.getElementById('text-input').value = select.value;
        if (select.value.includes('Dobrý')) {
          document.getElementById('voice-select').value = 'jirka';
        } else {
          if (document.getElementById('voice-select').value === 'jirka') {
            document.getElementById('voice-select').value = 'bass';
          }
        }
      }
    }

    async function synthesize(compare) {
      const text = document.getElementById('text-input').value.trim();
      if (!text) return;

      const voice = document.getElementById('voice-select').value;
      const speed = parseFloat(document.getElementById('speed-slider').value);

      const btnSynth = document.getElementById('btn-synth');
      const btnCompare = document.getElementById('btn-compare');
      const activeBtn = compare ? btnCompare : btnSynth;
      const origHtml = activeBtn.innerHTML;
      activeBtn.disabled = true;
      activeBtn.innerHTML = '<div class="spinner"></div> Syntetyzowanie...';

      const resultsArea = document.getElementById('results-area');
      resultsArea.style.display = 'grid';

      try {
        const resp = await fetch('/api/synthesize', {
          method: 'POST',
          headers: {'Content-Type': 'application/json'},
          body: JSON.stringify({ text, voice, speed, compare })
        });
        const data = await resp.json();

        if (data.error) {
          alert('Błąd syntezy: ' + data.error);
          return;
        }

        renderResults(data, compare);
      } catch (err) {
        alert('Błąd połączenia: ' + err);
      } finally {
        activeBtn.disabled = false;
        activeBtn.innerHTML = origHtml;
      }
    }

    function renderResults(data, compare) {
      const area = document.getElementById('results-area');
      if (!compare) {
        const tb = data.tbenni;
        area.innerHTML = `
          <div class="result-box tbenni">
            <div class="box-header">
              <div class="box-title">
                <span>🔊 TBenni Audio</span>
                <span class="speedup-pill">RTF: ${(tb.rtf).toFixed(3)} (30x w czasie rzecz.)</span>
              </div>
              <div class="telemetry">
                <span>G2P: <b>${tb.phonemize_time_ms.toFixed(3)} ms</b></span>
                <span>VITS: <b>${tb.synth_time_ms.toFixed(1)} ms</b></span>
                <span>Długość: <b>${tb.duration_s.toFixed(2)} s</b></span>
              </div>
            </div>
            <div class="phoneme-display">${escapeHtml(tb.phonemes)}</div>
            <audio controls autoplay src="data:audio/wav;base64,${tb.audio_b64}"></audio>
          </div>
        `;
      } else {
        const tb = data.tbenni;
        const es = data.espeak;
        const speedup = data.speedup;
        area.innerHTML = `
          <div style="grid-column: 1 / -1; display:flex; justify-content:space-between; align-items:center; padding: 0.5rem 0;">
            <h3 style="font-size:1.15rem; font-weight:600;">Wyniki A/B Testu</h3>
            <span class="speedup-pill">⚡ Front-end Speedup: ${speedup.toFixed(1)}x szybszy</span>
          </div>

          <div class="result-box tbenni">
            <div class="box-header">
              <div class="box-title" style="color:#34d399;">
                <span>✓ TBenni Native Front-End</span>
              </div>
              <div class="telemetry">
                <span>G2P: <b>${tb.phonemize_time_ms.toFixed(3)} ms</b></span>
                <span>VITS: <b>${tb.synth_time_ms.toFixed(1)} ms</b></span>
              </div>
            </div>
            <div class="phoneme-display">${escapeHtml(tb.phonemes)}</div>
            <audio controls src="data:audio/wav;base64,${tb.audio_b64}"></audio>
          </div>

          <div class="result-box espeak">
            <div class="box-header">
              <div class="box-title" style="color:#fbbf24;">
                <span>⚠️ eSpeak-NG Front-End</span>
              </div>
              <div class="telemetry">
                <span>G2P: <b>${es.phonemize_time_ms.toFixed(3)} ms</b></span>
                <span>VITS: <b>${es.synth_time_ms.toFixed(1)} ms</b></span>
              </div>
            </div>
            <div class="phoneme-display" style="color:#fbbf24;">${escapeHtml(es.phonemes)}</div>
            <audio controls src="data:audio/wav;base64,${es.audio_b64}"></audio>
          </div>
        `;
      }
    }

    function escapeHtml(str) {
      return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    }
  </script>
</body>
</html>
"""


class StudioServer(HTTPServer):
    def __init__(self, server_address, RequestHandlerClass):
        super().__init__(server_address, RequestHandlerClass)
        self.engines: dict = {}

    def get_engine(self, voice: str) -> tts.TTSEngine:
        now_mtime = None
        if voice in tts.VOICE_CATALOG and "local_path" in tts.VOICE_CATALOG[voice]:
            p = tts.Path(tts.VOICE_CATALOG[voice]["local_path"])
            if p.exists():
                now_mtime = p.stat().st_mtime

        if voice in self.engines:
            engine, cached_mtime = self.engines[voice]
            if now_mtime is None or now_mtime == cached_mtime:
                return engine
            print(f"Reloading updated voice model '{voice}' (new mtime: {now_mtime})...")

        print(f"Loading voice model '{voice}'...")
        engine = tts.TTSEngine(voice)
        self.engines[voice] = (engine, now_mtime)
        return engine


class StudioHandler(BaseHTTPRequestHandler):
    def log_message(self, format, *args):
        # Clean logging
        pass

    def do_GET(self):
        if self.path == "/" or self.path.startswith("/index"):
            self.send_response(200)
            self.send_header("Content-Type", "text/html; charset=utf-8")
            self.end_headers()
            self.wfile.write(HTML_TEMPLATE.encode("utf-8"))
        else:
            self.send_response(404)
            self.end_headers()

    def do_POST(self):
        if self.path == "/api/synthesize":
            content_length = int(self.headers.get("Content-Length", 0))
            body = self.rfile.read(content_length).decode("utf-8")
            try:
                req = json.loads(body)
                text = req.get("text", "")
                voice = req.get("voice", "bass")
                speed = float(req.get("speed", 1.0))
                compare = bool(req.get("compare", False))

                engine: tts.TTSEngine = self.server.get_engine(voice)

                if not compare:
                    res = engine.synthesize(text, speed=speed)
                    buf = io.BytesIO()
                    sf.write(buf, res.audio, res.sample_rate, format="WAV")
                    b64 = base64.b64encode(buf.getvalue()).decode("ascii")

                    out = {
                        "tbenni": {
                            "phonemes": res.phonemes,
                            "phonemize_time_ms": res.phonemize_time_ms,
                            "synth_time_ms": res.synth_time_ms,
                            "duration_s": res.duration_s,
                            "rtf": res.rtf,
                            "audio_b64": b64,
                        }
                    }
                else:
                    comp = engine.compare_with_espeak(text)
                    tb_res = comp["tbenni"]
                    es_res = comp["espeak"]

                    buf_tb = io.BytesIO()
                    sf.write(buf_tb, tb_res.audio, tb_res.sample_rate, format="WAV")
                    b64_tb = base64.b64encode(buf_tb.getvalue()).decode("ascii")

                    buf_es = io.BytesIO()
                    sf.write(buf_es, es_res.audio, es_res.sample_rate, format="WAV")
                    b64_es = base64.b64encode(buf_es.getvalue()).decode("ascii")

                    out = {
                        "speedup": comp["speedup"],
                        "tbenni": {
                            "phonemes": tb_res.phonemes,
                            "phonemize_time_ms": tb_res.phonemize_time_ms,
                            "synth_time_ms": tb_res.synth_time_ms,
                            "duration_s": tb_res.duration_s,
                            "rtf": tb_res.rtf,
                            "audio_b64": b64_tb,
                        },
                        "espeak": {
                            "phonemes": es_res.phonemes,
                            "phonemize_time_ms": es_res.phonemize_time_ms,
                            "synth_time_ms": es_res.synth_time_ms,
                            "duration_s": es_res.duration_s,
                            "rtf": es_res.rtf,
                            "audio_b64": b64_es,
                        }
                    }

                resp_bytes = json.dumps(out).encode("utf-8")
                self.send_response(200)
                self.send_header("Content-Type", "application/json")
                self.send_header("Content-Length", str(len(resp_bytes)))
                self.end_headers()
                self.wfile.write(resp_bytes)

            except Exception as e:
                err_bytes = json.dumps({"error": str(e)}).encode("utf-8")
                self.send_response(500)
                self.send_header("Content-Type", "application/json")
                self.send_header("Content-Length", str(len(err_bytes)))
                self.end_headers()
                self.wfile.write(err_bytes)
        else:
            self.send_response(404)
            self.end_headers()


def run_studio(port: int = 8080, host: str = "127.0.0.1"):
    """Launch the TBenni Neural Speech Studio web interface."""
    server = StudioServer((host, port), StudioHandler)
    url = f"http://{host}:{port}"
    print("=" * 70)
    print(f"🎙️  TBenni Neural Speech Studio running at: {url}")
    print("   Open this URL in your web browser to test voices interactively!")
    print("   Press Ctrl+C to terminate.")
    print("=" * 70)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nStudio server stopped.")
    finally:
        server.server_close()


def main() -> None:
    """Console entry point for ``tbenni-web``."""
    import argparse

    parser = argparse.ArgumentParser(description="TBenni Neural Speech Studio")
    parser.add_argument("--port", type=int, default=8080, help="Port to listen on (default 8080)")
    parser.add_argument("--host", type=str, default="127.0.0.1", help="Host address (default 127.0.0.1)")
    args = parser.parse_args()
    run_studio(port=args.port, host=args.host)


if __name__ == "__main__":
    main()
