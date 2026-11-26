//! Hand-written base phonotactic rules for Ukrainian (the TBenni "base mapper").
//!
//! Maintained by hand, NOT generated. The distillation pipeline compares the output of these
//! rules against the neural teacher, one grapheme segment at a time, and compiles the learned
//! corrections into `langs/uk.rs`.

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

        let is_iotated = matches!(c, 'я' | 'є' | 'ю' | 'ї');
        let is_post_vowel = prev.is_none_or(|p| {
            matches!(
                p,
                'а' | 'о' | 'у' | 'е' | 'и' | 'і' | 'я' | 'є' | 'ю' | 'ї' | 'ь' | '\''
            )
        });
        if is_iotated && is_post_vowel {
            match c {
                'я' => result.push_str("ja"),
                'є' => result.push_str("je"),
                'ю' => result.push_str("ju"),
                'ї' => result.push_str("ji"),
                _ => {}
            }
            i += 1;
            continue;
        }
        match c {
            'а' => result.push('a'),
            'б' => result.push('b'),
            'в' => result.push('w'),
            'г' => result.push('ɦ'),
            'ґ' => result.push('ɡ'),
            'д' => result.push('d'),
            'е' => result.push('e'),
            'є' => result.push('e'),
            'ж' => result.push('ʒ'),
            'з' => result.push('z'),
            'и' => result.push('ɪ'),
            'і' => result.push('i'),
            'ї' => result.push_str("ji"),
            'й' => result.push('j'),
            'к' => result.push('k'),
            'л' => result.push('l'),
            'м' => result.push('m'),
            'н' => result.push('n'),
            'о' => result.push('o'),
            'п' => result.push('p'),
            'р' => result.push('r'),
            'с' => result.push('s'),
            'т' => result.push('t'),
            'у' => result.push('u'),
            'ф' => result.push('f'),
            'х' => result.push('x'),
            'ц' => result.push_str("ts"),
            'ч' => result.push_str("tʃ"),
            'ш' => result.push('ʃ'),
            'щ' => result.push_str("ʃtʃ"),
            'ь' => result.push('ʲ'),
            'ю' => result.push('u'),
            'я' => result.push('a'),
            '\'' => {}
            _ => result.push(c),
        }
        i += 1;
        continue;
    }
    i
}
