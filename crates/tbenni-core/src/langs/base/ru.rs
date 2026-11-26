//! Hand-written base phonotactic rules for Russian (the TBenni "base mapper").
//!
//! Maintained by hand, NOT generated. The distillation pipeline compares the output of these
//! rules against the neural teacher, one grapheme segment at a time, and compiles the learned
//! corrections into `langs/ru.rs`.

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

        // Iotated vowels at word start or after vowel/hard/soft signs
        let is_iotated = matches!(c, 'я' | 'е' | 'ё' | 'ю');
        let is_post_vowel = prev.is_none_or(|p| {
            matches!(
                p,
                'а' | 'о' | 'у' | 'ы' | 'э' | 'я' | 'е' | 'ё' | 'ю' | 'и' | 'ь' | 'ъ'
            )
        });
        if is_iotated && is_post_vowel {
            match c {
                'я' => result.push_str("ja"),
                'е' => result.push_str("je"),
                'ё' => result.push_str("jo"),
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
                Some('п' | 'ф' | 'к' | 'т' | 'ш' | 'с' | 'х' | 'ц' | 'ч' | 'щ')
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
                    result.push('ʂ');
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
        // Cyrillic characters mapping
        match c {
            'а' => result.push('a'),
            'б' => result.push('b'),
            'в' => result.push('v'),
            'г' => result.push('ɡ'),
            'д' => result.push('d'),
            'е' => result.push('e'),
            'ё' => result.push('o'),
            'ж' => result.push('ʐ'),
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
            'ч' => result.push_str("tɕ"),
            'ш' => result.push('ʂ'),
            'щ' => result.push_str("ɕː"),
            'ъ' => {}
            'ы' => result.push('ɨ'),
            'ь' => result.push('ʲ'),
            'э' => result.push('e'),
            'ю' => result.push('u'),
            'я' => result.push('a'),
            _ => result.push(c),
        }
        i += 1;
        continue;
    }
    i
}
