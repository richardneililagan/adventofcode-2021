#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|d| d.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day7, part1, original)]
pub fn solver_part1(input: &[i32]) -> i32 {
    let cmin = input.iter().min().unwrap();
    let cmax = input.iter().max().unwrap();

    let mut minimumfuel = i32::MAX;

    for i in *cmin..*cmax {
        let s: i32 = input.iter().map(|n| (i - n).abs()).sum();
        if s < minimumfuel {
            minimumfuel = s;
        }
    }

    minimumfuel
}

#[aoc(day7, part2, original)]
pub fn solver_part2(input: &[i32]) -> u128 {
    let cmin = input.iter().min().unwrap();
    let cmax = input.iter().max().unwrap();

    let mut minimumfuel = u128::MAX;

    for i in *cmin..*cmax {
        let s: u128 = input
            .iter()
            .map(|n| {
                let steps: u128 = (i - n).abs() as u128;
                let total_fuel: u128 = (1..=steps).sum();

                total_fuel
            })
            .sum();

        if s < minimumfuel {
            minimumfuel = s;
        }
    }

    minimumfuel
}

// :: ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input: &str = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(solver_part1(&generator(input)), 37);
    }

    #[test]
    fn example_2() {
        let input: &str = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(solver_part2(&generator(input)), 168);
    }
}
