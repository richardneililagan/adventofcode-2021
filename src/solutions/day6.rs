use std::collections::HashMap;

struct OffspringCounter {
    memo: HashMap<(u8, usize), usize>,
}

impl OffspringCounter {
    fn new() -> OffspringCounter {
        OffspringCounter {
            memo: HashMap::new(),
        }
    }

    fn count_offspring(&mut self, days_left: u8, total_time: usize) -> usize {
        // :: If we have less total time left than the number of days before a new
        //    offspring is created, then we just return 1 (for this fish).
        // :: There's a +1 because the fish don't multiply on day 0;
        //    they multiply on the day after day 0
        if usize::from(days_left) + 1 > total_time {
            return 1;
        }

        // :: If we've already encountered this pair before, just return the memoized value.
        if let Some(offspring) = self.memo.get(&(days_left, total_time)) {
            return *offspring;
        }

        // :: Else, we invoke the absolute power of recursion! /evil laugh
        let this_fish = self.count_offspring(6, total_time - usize::from(days_left) - 1);
        let new_fish = self.count_offspring(8, total_time - usize::from(days_left) - 1);

        let total_offspring = this_fish + new_fish;

        // :: Don't forget to cache the result.
        self.memo.insert((days_left, total_time), total_offspring);

        total_offspring
    }
}

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<u8> {
    input
        .split(',')
        .map(|d| d.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
}

#[aoc(day6, part1, original)]
pub fn solver_part1(input: &[u8]) -> usize {
    const NUMBER_OF_DAYS: usize = 80;
    let mut counter = OffspringCounter::new();

    input
        .iter()
        .map(|d| counter.count_offspring(*d, NUMBER_OF_DAYS))
        .sum()
}

#[aoc(day6, part2, original)]
pub fn solver_part2(input: &[u8]) -> usize {
    const NUMBER_OF_DAYS: usize = 256;
    let mut counter = OffspringCounter::new();

    input
        .iter()
        .map(|d| counter.count_offspring(*d, NUMBER_OF_DAYS))
        .sum()
}

// :: ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_works_1() {
        let mut counter = OffspringCounter::new();
        let offspring = counter.count_offspring(3, 7);

        assert_eq!(offspring, 2);
    }

    #[test]
    fn example_1_18days() {
        let mut counter = OffspringCounter::new();
        let input = "3,4,3,1,2";

        let offspring: usize = generator(input)
            .iter()
            .map(|d| counter.count_offspring(*d, 18))
            .sum();

        assert_eq!(offspring, 26);
    }
}
