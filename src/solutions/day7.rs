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

    let mut minimumfuel = cmax * input.len() as i32;

    for i in *cmin..*cmax {
        let s: i32 = input.iter().map(|n| (i - n).abs()).sum();
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
}
