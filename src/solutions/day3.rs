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
}
