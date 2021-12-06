#[derive(Debug, Clone)]
pub struct Board {
    slots: Vec<(bool, i32)>,
}

impl Board {
    fn is_bingo(&self) -> bool {
        let c: Vec<bool> = self.slots.iter().map(|&(b, _)| b).collect();

        {
            // :: Yeahhhhh, we can definitely do this dynamically instead,
            //    but I'm feeling lazy right now
            (c[0] && c[1] && c[2] && c[3] && c[4])
                || (c[5] && c[6] && c[7] && c[8] && c[9])
                || (c[10] && c[11] && c[12] && c[13] && c[14])
                || (c[15] && c[16] && c[17] && c[18] && c[19])
                || (c[20] && c[21] && c[22] && c[23] && c[24])
                || (c[0] && c[5] && c[10] && c[15] && c[20])
                || (c[1] && c[6] && c[11] && c[16] && c[21])
                || (c[2] && c[7] && c[12] && c[17] && c[22])
                || (c[3] && c[8] && c[13] && c[18] && c[23])
                || (c[4] && c[9] && c[14] && c[19] && c[24])
        }
    }

    fn call_number(&mut self, num: i32) {
        self.slots
            .iter_mut()
            .filter(|&&mut (_, n)| n == num)
            .for_each(|mut slot| slot.0 = true);
    }

    fn uncalled_value(&self) -> i32 {
        self.slots
            .iter()
            .filter(|&&(b, _)| !b)
            .map(|(_, n)| n)
            .sum()
    }
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> (Vec<i32>, Vec<Board>) {
    let mut lines = input.lines().map(|line| line.trim()).collect::<Vec<&str>>();
    lines.retain(|line| !line.is_empty());

    let called_numbers: Vec<i32> = lines[0]
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let board_numbers: Vec<Vec<i32>> = lines[1..]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|d| d.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
        .chunks(5)
        .map(|w| w.concat())
        .collect();

    let boards: Vec<Board> = board_numbers
        .iter()
        .map(|nums| Board {
            slots: nums
                .iter()
                .map(|n| (false, *n))
                .collect::<Vec<(bool, i32)>>(),
        })
        .collect();

    (called_numbers, boards)
}

#[aoc(day4, part1, original)]
pub fn solver_part1((called_nums, b): &(Vec<i32>, Vec<Board>)) -> i32 {
    let mut boards = b.clone();

    for &n in called_nums {
        boards.iter_mut().for_each(|b| b.call_number(n));
        if let Some(b) = boards.iter().find(|&board| board.is_bingo()) {
            return b.uncalled_value() * n;
        }
    }

    unreachable!();
}

#[aoc(day4, part2, original)]
pub fn solver_part2((called_nums, b): &(Vec<i32>, Vec<Board>)) -> i32 {
    let mut boards = b.clone();

    for &n in called_nums {
        boards.iter_mut().for_each(|b| b.call_number(n));
        if boards.len() > 1 {
            boards.retain(|b| !b.is_bingo());
        } else if boards[0].is_bingo() {
            return boards[0].uncalled_value() * n;
        }
    }

    unreachable!();
}

// :: ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

                        22 13 17 11  0
                         8  2 23  4 24
                        21  9 14 16  7
                         6 10  3 18  5
                         1 12 20 15 19
      
                         3 15  0  2 22
                         9 18 13 17  5
                        19  8  7 25 23
                        20 11 10 24  4
                        14 21 16 12  6
      
                        14 21 17 24  4
                        10 16 15  9 19
                        18  8 23 26 20
                        22 11 13  6  5
                         2  0 12  3  7";

        assert_eq!(solver_part1(&generator(input)), 4512);
    }

    #[test]
    fn example_2() {
        let input: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

                        22 13 17 11  0
                         8  2 23  4 24
                        21  9 14 16  7
                         6 10  3 18  5
                         1 12 20 15 19
      
                         3 15  0  2 22
                         9 18 13 17  5
                        19  8  7 25 23
                        20 11 10 24  4
                        14 21 16 12  6
      
                        14 21 17 24  4
                        10 16 15  9 19
                        18  8 23 26 20
                        22 11 13  6  5
                         2  0 12  3  7";

        assert_eq!(solver_part2(&generator(input)), 1924);
    }
}
