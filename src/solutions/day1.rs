#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<i32> {
    input
        .lines()
        // .map(|line| line.trim())
        .map(|line| i32::from_str_radix(line.trim(), 10).unwrap_or(0))
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

#[aoc(day1, part1, windows_fold)]
pub fn solver_part1_fold(input: &Vec<i32>) -> u32 {
    input
        .windows(2)
        .fold(0, |acc, x| if x[1] > x[0] { acc + 1 } else { acc })
}

#[aoc(day1, part2)]
pub fn solver_part2(input: &Vec<i32>) -> u32 {
    let sums: Vec<i32> = (0..(input.len() - 2))
        .map(|i| input[i] + input[i + 1] + input[i + 2])
        .collect();

    sums.iter()
        .enumerate()
        .map(|(index, _)| match index {
            0 => 0,
            x => {
                if sums[x] > sums[x - 1] {
                    1 as u32
                } else {
                    0 as u32
                }
            }
        })
        .sum()
}

#[aoc(day1, part2, windows_fold)]
pub fn solver_part2_fold(input: &Vec<i32>) -> u32 {
    input
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect::<Vec<i32>>()
        .windows(2)
        .fold(0, |acc, x| if x[1] > x[0] { acc + 1 } else { acc })
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

    #[test]
    fn example_2() {
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

        assert_eq!(solver_part2(&generator(input)), 5);
    }
}
