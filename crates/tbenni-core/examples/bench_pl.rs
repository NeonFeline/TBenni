//! Single-thread throughput of the Polish engine.
//! Usage: cargo run --release -p tbenni-core --example bench_pl -- <wordlist (first column)>
use std::time::Instant;

fn main() {
    let path = std::env::args().nth(1).expect("word list path");
    let text = std::fs::read_to_string(path).expect("read word list");
    let words: Vec<&str> = text
        .lines()
        .filter_map(|l| l.split(|c| c == '\t' || c == ' ').next())
        .filter(|w| !w.is_empty())
        .collect();
    let mut best = f64::MAX;
    let mut sink = 0usize;
    for _ in 0..7 {
        let t0 = Instant::now();
        for w in &words {
            sink += tbenni_core::phonemize(w).len();
        }
        best = best.min(t0.elapsed().as_secs_f64());
    }
    println!(
        "{} words, best of 7: {:.3}s -> {:.0} words/s (sink {})",
        words.len(),
        best,
        words.len() as f64 / best,
        sink
    );
}
