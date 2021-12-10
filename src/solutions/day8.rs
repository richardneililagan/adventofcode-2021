type Digit = Vec<char>;

pub struct SegmentDisplay {
    digits: Vec<Digit>, // Vec<Vec<char>>
    output: Vec<Digit>, // Vec<Vec<char>>
}

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
}
