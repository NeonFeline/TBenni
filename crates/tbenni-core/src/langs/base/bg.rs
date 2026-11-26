//! Hand-written base phonotactic rules for Bulgarian (the TBenni "base mapper").
//!
//! Maintained by hand, NOT generated. The distillation pipeline compares the output of these
//! rules against the neural teacher, one grapheme segment at a time, and compiles the learned
//! corrections into `langs/bg.rs`.

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

        // Iotated я, ю after vowels or at word start
        let is_iotated = matches!(c, 'я' | 'ю');
        let is_post_vowel =
            prev.is_none_or(|p| matches!(p, 'а' | 'о' | 'у' | 'е' | 'и' | 'ъ' | 'я' | 'ю' | 'ь'));
        if is_iotated && is_post_vowel {
            match c {
                'я' => result.push_str("ja"),
                'ю' => result.push_str("ju"),
                _ => {}
            }
            i += 1;
            continue;
        }
        // Voicing assimilation & final devoicing
        let is_end_or_voiceless = next.is_none()
            || matches!(
                next,
                Some('п' | 'ф' | 'к' | 'т' | 'ш' | 'с' | 'х' | 'ц' | 'ч')
            );
        if is_end_or_voiceless {
            match c {
                'б' => {
                    result.push('p');
                    i += 1;
                    continue;
                }
                'в' => {
                    result.push('f');
                    i += 1;
                    continue;
                }
                'г' => {
                    result.push('k');
                    i += 1;
                    continue;
                }
                'д' => {
                    result.push('t');
                    i += 1;
                    continue;
                }
                'ж' => {
                    result.push('ʃ');
                    i += 1;
                    continue;
                }
                'з' => {
                    result.push('s');
                    i += 1;
                    continue;
                }
                _ => {}
            }
        }
        // Softening before ь, я, ю
        let is_soft = next.is_some_and(|n| matches!(n, 'ь' | 'я' | 'ю'));
        if is_soft
            && matches!(
                c,
                'б' | 'в'
                    | 'г'
                    | 'д'
                    | 'з'
                    | 'к'
                    | 'л'
                    | 'м'
                    | 'н'
                    | 'п'
                    | 'р'
                    | 'с'
                    | 'т'
                    | 'ф'
                    | 'х'
                    | 'ц'
            )
        {
            match c {
                'б' => result.push_str("bʲ"),
                'в' => result.push_str("vʲ"),
                'г' => result.push_str("ɡʲ"),
                'д' => result.push_str("dʲ"),
                'з' => result.push_str("zʲ"),
                'к' => result.push_str("kʲ"),
                'л' => result.push_str("lʲ"),
                'м' => result.push_str("mʲ"),
                'н' => result.push_str("nʲ"),
                'п' => result.push_str("pʲ"),
                'р' => result.push_str("rʲ"),
                'с' => result.push_str("sʲ"),
                'т' => result.push_str("tʲ"),
                'ф' => result.push_str("fʲ"),
                'х' => result.push_str("xʲ"),
                'ц' => result.push_str("tsʲ"),
                _ => result.push(c),
            }
            if next == Some('ь') {
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }
        // General character mapping
        match c {
            'а' => result.push('a'),
            'б' => result.push('b'),
            'в' => result.push('v'),
            'г' => result.push('ɡ'),
            'д' => result.push('d'),
            'е' => result.push('e'),
            'ж' => result.push('ʒ'),
            'з' => result.push('z'),
            'и' => result.push('i'),
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
            'щ' => result.push_str("ʃt"),
            'ъ' => result.push('ɤ'),
            'ь' => result.push('ʲ'),
            'ю' => result.push('u'),
            'я' => result.push('a'),
            _ => result.push(c),
        }
        i += 1;
        continue;
    }
    i
}
