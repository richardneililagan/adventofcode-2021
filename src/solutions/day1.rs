#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| i32::from_str_radix(line, 10).unwrap_or(0))
        .collect()
}

#[aoc(day1, part1)]
pub fn solver_part1(input: &Vec<i32>) -> u32 {
    input
        .iter()
        .enumerate()
        .map(|(index, _)| match index {
            0 => 0,
            x => {
                if input[x] > input[x - 1] {
                    1 as u32
                } else {
                    0 as u32
                }
            }
        })
        .sum()
}

// :: ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "199
                     200
                     208
                     210
                     200
                     207
                     240
                     269
                     260
                     263";

        assert_eq!(solver_part1(&generator(input)), 7);
    }
}
