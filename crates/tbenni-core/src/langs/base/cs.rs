//! Hand-written base phonotactic rules for Czech (the TBenni "base mapper").
//!
//! Maintained by hand, NOT generated. The distillation pipeline compares the output of these
//! rules against the neural teacher, one grapheme segment at a time, and compiles the learned
//! corrections into `langs/cs.rs`.

/// Emits the phones for the grapheme segment starting at `i` into `result` and returns the
/// index just past the segment.
///
/// The rule body is written as a loop body (`i += n; continue;`). It runs exactly once because
/// every rule advances `i`.
#[inline]
#[allow(unused_variables)]
pub fn step(chars: &[char], i: usize, result: &mut String) -> usize {
    let start = i;
    let mut i = i;
    while i == start {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        let next2 = chars.get(i + 2).copied();
        let prev = if i > 0 { Some(chars[i - 1]) } else { None };

        // Digraph ch -> [x]
        if c == 'c' && next == Some('h') {
            result.push('x');
            i += 2;
            continue;
        }
        // dě, tě, ně palatalization -> [ɟɛ], [cɛ], [ɲɛ]
        if c == 'd' && next == Some('ě') {
            result.push_str("ɟɛ");
            i += 2;
            continue;
        }
        if c == 't' && next == Some('ě') {
            result.push_str("cɛ");
            i += 2;
            continue;
        }
        if c == 'n' && next == Some('ě') {
            result.push_str("ɲɛ");
            i += 2;
            continue;
        }
        // mě -> [mɲɛ]
        if c == 'm' && next == Some('ě') {
            result.push_str("mɲɛ");
            i += 2;
            continue;
        }
        // di, ti, ni -> [ɟɪ], [cɪ], [ɲɪ]
        if c == 'd' && next == Some('i') {
            result.push_str("ɟɪ");
            i += 2;
            continue;
        }
        if c == 't' && next == Some('i') {
            result.push_str("cɪ");
            i += 2;
            continue;
        }
        if c == 'n' && next == Some('i') {
            result.push_str("ɲɪ");
            i += 2;
            continue;
        }
        // Czech characters mapping
        match c {
            'č' => result.push_str("tʂ"),
            'š' => result.push('ʂ'),
            'ž' => result.push('ʐ'),
            'ř' => result.push_str("r̝"),
            'ď' => result.push('ɟ'),
            'ť' => result.push('c'),
            'ň' => result.push('ɲ'),
            'c' => result.push_str("ts"),
            'h' => result.push('ɦ'),
            'y' => result.push('ɪ'),
            'ý' => result.push_str("iː"),
            'á' => result.push_str("aː"),
            'é' => result.push_str("ɛː"),
            'í' => result.push_str("iː"),
            'ó' => result.push_str("oː"),
            'ú' | 'ů' => result.push_str("uː"),
            'e' => result.push('ɛ'),
            'i' => result.push('ɪ'),
            _ => result.push(c),
        }
        i += 1;
    }
    i
}
