type Matrix<T> = Vec<Vec<T>>;

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Matrix<char> {
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| line.chars().collect())
        .collect()
}

#[aoc(day3, part1, original)]
pub fn solver_part1(input: &[Vec<char>]) -> u32 {
    let num_length = input[0].len(); // yeah, this is super unsafe

    let gamma_string: String = (0..num_length)
        .map(|index| {
            let count = input
                .iter()
                .map(|line| line[index])
                .fold((0, 0), |acc, d| match d {
                    '0' => (acc.0 + 1, acc.1),
                    '1' => (acc.0, acc.1 + 1),
                    _ => unreachable!(),
                });

            if count.0 > count.1 {
                '0'
            } else {
                '1'
            }
        })
        .collect();

    let epsilon_string: String = gamma_string
        .chars()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => unreachable!(),
        })
        .collect();

    let gamma: u32 = u32::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon: u32 = u32::from_str_radix(&epsilon_string, 2).unwrap();

    gamma * epsilon
}

#[aoc(day3, part2, original)]
pub fn solver_part2(input: &[Vec<char>]) -> u32 {
    let num_length = input[0].len(); // yeah, again, super unsafe
    let mut o2_reports = input.to_vec();
    let mut co2_reports = input.to_vec();

    for i in 0..num_length {
        let majority_digit_count = o2_reports
            .iter()
            .map(|line| line[i])
            .fold((0, 0), |acc, d| match d {
                '0' => (acc.0 + 1, acc.1),
                '1' => (acc.0, acc.1 + 1),
                _ => unreachable!(),
            });

        let majority_digit = if majority_digit_count.0 > majority_digit_count.1 {
            '0'
        } else {
            '1'
        };

        o2_reports.retain(|line| line[i] == majority_digit);
        if o2_reports.len() == 1 {
            break;
        }
    }

    for i in 0..num_length {
        let digit_count = co2_reports
            .iter()
            .map(|line| line[i])
            .fold((0, 0), |acc, d| match d {
                '0' => (acc.0 + 1, acc.1),
                '1' => (acc.0, acc.1 + 1),
                _ => unreachable!(),
            });

        let minority_digit = if digit_count.1 < digit_count.0 {
            '1'
        } else {
            '0'
        };

        co2_reports.retain(|line| line[i] == minority_digit);
        if co2_reports.len() == 1 {
            break;
        }
    }

    let o2_rating_string: String = o2_reports[0].iter().collect();
    let co2_rating_string: String = co2_reports[0].iter().collect();

    let o2_rating = u32::from_str_radix(&o2_rating_string, 2).unwrap();
    let co2_rating = u32::from_str_radix(&co2_rating_string, 2).unwrap();

    o2_rating * co2_rating
}

// :: ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        //
        let input = "00100
                      11110
                      10110
                      10111
                      10101
                      01111
                      00111
                      11100
                      10000
                      11001
                      00010
                      01010";

        assert_eq!(solver_part1(&generator(input)), 198);
    }

    #[test]
    fn example_2() {
        //
        let input = "00100
                      11110
                      10110
                      10111
                      10101
                      01111
                      00111
                      11100
                      10000
                      11001
                      00010
                      01010";

        assert_eq!(solver_part2(&generator(input)), 230);
    }
}
