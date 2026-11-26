//! Hand-written base phonotactic rules for Croatian (the TBenni "base mapper").
//!
//! Maintained by hand, NOT generated. The distillation pipeline compares the output of these
//! rules against the neural teacher, one grapheme segment at a time, and compiles the learned
//! corrections into `langs/hr.rs`.

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

        // Digraph dž -> [dʐ]
        if c == 'd' && next == Some('ž') {
            result.push_str("dʐ");
            i += 2;
            continue;
        }
        // Digraph lj -> [ʎ]
        if c == 'l' && next == Some('j') {
            result.push('ʎ');
            i += 2;
            continue;
        }
        // Digraph nj -> [ɲ]
        if c == 'n' && next == Some('j') {
            result.push('ɲ');
            i += 2;
            continue;
        }
        match c {
            'č' => result.push_str("tʂ"),
            'ć' => result.push_str("tɕ"),
            'đ' => result.push_str("dʑ"),
            'š' => result.push('ʂ'),
            'ž' => result.push('ʐ'),
            'c' => result.push_str("ts"),
            'h' => result.push('x'),
            _ => result.push(c),
        }
        i += 1;
        continue;
    }
    i
}
