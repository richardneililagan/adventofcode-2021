use std::collections::{HashMap, HashSet};

type Digit = HashSet<char>;

pub struct SegmentDisplay {
    digits: Vec<Digit>, // Vec<Vec<char>>
    output: Vec<Digit>, // Vec<Vec<char>>
}

// Really sorry to have to break this apart into vec of vec of ...,
// but `cargo-aoc` has a bug wherein we can't return a type that has a lifetime from
// the marked generator function. :(
// https://github.com/gobanos/cargo-aoc/issues/20
#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<SegmentDisplay> {
    input
        .lines()
        .map(|line| {
            let mut segments = line.split('|').map(|s| s.trim());
            (segments.next().unwrap(), segments.next().unwrap())
        })
        .map(|(s, t)| {
            (
                s.split(' ').map(|a| a.chars().collect::<Digit>()).collect(),
                t.split(' ').map(|a| a.chars().collect::<Digit>()).collect(),
            )
        })
        .map(|(d, o)| SegmentDisplay {
            digits: d,
            output: o,
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solver_part1(input: &[SegmentDisplay]) -> usize {
    input.iter().fold(0, |acc, display| {
        let valid_digits = display
            .output
            .iter()
            .filter(|digits| matches!(digits.len(), 2 | 3 | 4 | 7))
            .count();

        acc + valid_digits
    })
}

/// Decodes a single SegmentDisplay, and returns the decoded output value.
fn display_decoder(display: &SegmentDisplay) -> usize {
    let mut known_segments: HashMap<char, Option<&char>> = HashMap::from([
        ('a', None),
        ('b', None),
        ('c', None),
        ('d', None),
        ('e', None),
        ('f', None),
        ('g', None),
    ]);

    //  Let's grab some known numbers first:
    //  - 4 segments = 4
    //  - 7 segments = 8

    let code_4 = display.digits.iter().find(|d| d.len() == 4).unwrap();
    let code_7 = display.digits.iter().find(|d| d.len() == 3).unwrap();

    //  The (a) segment is the difference between the 4 and 7:
    // let a_segment = code_4.symmetric_difference(code_7).find(|_| true);
    let a_segment = code_7.difference(code_4).find(|_| true);
    known_segments.insert('a', a_segment);

    //  Some of the segments we can determine by the number of times they come up
    //  in the test set --- let's count those instances now.
    let mut segment_counts: HashMap<&char, usize> = HashMap::new();
    for digit in display.digits.iter() {
        for d in digit {
            let count = segment_counts.entry(d).or_insert(0);
            *count += 1;
        }
    }

    //  Segment counts 4, 6, and 9 are uniquely identifiable:
    //
    //       (a)
    //  <6>        |
    //       ---
    //  <4>       <9>
    //       ---

    let b_segment = segment_counts
        .iter()
        .find(|&(_, count)| *count == 6)
        .map(|(key, _)| *key);

    let e_segment = segment_counts
        .iter()
        .find(|&(_, count)| *count == 4)
        .map(|(key, _)| *key);

    let f_segment = segment_counts
        .iter()
        .find(|&(_, count)| *count == 9)
        .map(|(key, _)| *key);

    known_segments.insert('b', b_segment);
    known_segments.insert('e', e_segment);
    known_segments.insert('f', f_segment);

    //  Segment (a) and (c) appear 8 times each, but we already know what maps to
    //  (a) at this point, so the other one must be (c).
    let c_segment = segment_counts
        .iter()
        .filter(|&(_, count)| *count == 8)
        .find(|&(key, _)| {
            let a = a_segment.unwrap();
            **key != *a
        })
        // .find(|&(key, _)| Some(*key) != a_segment)
        .map(|(key, _)| *key);

    known_segments.insert('c', c_segment);

    //  We already know segments (b), (c), and (f) now, so we can intersect that
    //  with the characters for known number 4 to determine segment (d).
    let bcd = [b_segment, c_segment, f_segment].map(|s| match s {
        Some(c) => *c,
        _ => unreachable!(),
    });

    let bcd_hs = HashSet::from(bcd);
    let d_segment = bcd_hs.symmetric_difference(code_4).find(|_| true);

    known_segments.insert('d', d_segment);

    //  Finally, segment (g) is the only one unidentified at this point.
    let mut candidates = "abcdefg".chars().collect::<Vec<char>>();
    for value in known_segments.values().flatten() {
        candidates.retain(|d| d != *value);
    }

    let g_segment = known_segments
        .iter()
        .find(|(_, v)| matches!(v, None))
        .unwrap();

    let g_segment_key = *g_segment.0;
    *known_segments.get_mut(&g_segment_key).unwrap() = Some(&candidates[0]);

    //  We can now build the comparison / translated digits.
    let originals = vec![
        "abcefg",  // 0
        "cf",      // 1
        "acdeg",   // 2,
        "acdfg",   // 3
        "bcdf",    // 4
        "abdfg",   // 5
        "abdefg",  // 6
        "acf",     // 7
        "abcdefg", // 8
        "abcdfg",  // 9
    ];

    let translated: Vec<HashSet<char>> = originals
        .iter()
        .map(|s| s.chars()) //
        .map(|chars| {
            let cipher: HashSet<char> = chars
                .map(|c| known_segments.get(&c).unwrap().unwrap())
                .fold(HashSet::new(), |mut acc, c| {
                    acc.insert(*c);
                    acc
                });

            cipher
        })
        .collect();

    let output = display
        .output
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, hs)| {
            let value = translated
                .iter()
                .position(|ths| ths.symmetric_difference(hs).count() == 0)
                .unwrap();

            acc + value * (10usize.pow(i.try_into().unwrap()))
        });

    dbg!(&translated);
    dbg!(&known_segments);

    output
}

#[aoc(day8, part2)]
pub fn solver_part2(input: &[SegmentDisplay]) -> usize {
    input
        .iter()
        .fold(0, |acc, display| acc + display_decoder(display))
}

// :: ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
             edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
             fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
             fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
             aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
             fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
             dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
             bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
             egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
             gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(solver_part1(&generator(input)), 26);
    }

    #[test]
    fn example_2() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
             edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
             fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
             fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
             aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
             fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
             dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
             bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
             egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
             gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(solver_part2(&generator(input)), 61229);
    }
}
