/// Position for part 1
struct Position {
    horizon: i32,
    depth: i32,
}

/// Position for part 2
struct Position2 {
    horizon: i32,
    depth: i32,
    aim: i32,
}

pub enum Instruction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.trim().split(' '))
        .map(|mut split| {
            (
                split.next().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .map(|pair| match pair {
            ("forward", n) => Instruction::Forward(n),
            ("up", n) => Instruction::Up(n),
            ("down", n) => Instruction::Down(n),
            _ => unreachable!(),
        })
        .collect()
}

#[aoc(day2, part1, original)]
pub fn solver_part1(input: &[Instruction]) -> i32 {
    let initial = Position {
        horizon: 0,
        depth: 0,
    };

    let end = input
        .iter()
        .fold(initial, |acc, instruction| match instruction {
            Instruction::Forward(n) => Position {
                horizon: acc.horizon + n,
                ..acc
            },
            Instruction::Up(n) => Position {
                depth: acc.depth - n,
                ..acc
            },
            Instruction::Down(n) => Position {
                depth: acc.depth + n,
                ..acc
            },
        });

    end.horizon * end.depth
}

#[aoc(day2, part2, original)]
pub fn solver_part2(input: &[Instruction]) -> i32 {
    let initial = Position2 {
        horizon: 0,
        depth: 0,
        aim: 0,
    };

    let end = input
        .iter()
        .fold(initial, |acc, instruction| match instruction {
            Instruction::Up(n) => Position2 {
                aim: acc.aim - n,
                ..acc
            },
            Instruction::Down(n) => Position2 {
                aim: acc.aim + n,
                ..acc
            },
            Instruction::Forward(n) => Position2 {
                horizon: acc.horizon + n,
                depth: acc.depth + (acc.aim * n),
                ..acc
            },
        });

    end.horizon * end.depth
}

// :: ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "forward 5
                 down 5
                 forward 8
                 up 3
                 down 8
                 forward 2";

        assert_eq!(solver_part1(&generator(input)), 150);
    }

    #[test]
    fn example_2() {
        let input = "forward 5
                 down 5
                 forward 8
                 up 3
                 down 8
                 forward 2";

        assert_eq!(solver_part2(&generator(input)), 900);
    }
}
