use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point(i32, i32);

#[derive(Debug)]
pub struct Segment(Point, Point);

impl Segment {
    fn is_diagonal(&self) -> bool {
        let p1 = self.0;
        let p2 = self.1;

        (p1.0 != p2.0) && (p1.1 != p2.1)
    }

    /// Get the points that are on this line segment, including the endpoints.
    fn get_points(&self) -> Vec<Point> {
        match self {
            Segment(Point(x1, m), Point(x2, n)) if x1 == x2 => {
                let yrange = if m < n {
                    (*m..=*n).collect::<Vec<i32>>()
                } else {
                    (*n..=*m).rev().collect::<Vec<i32>>()
                };

                yrange.iter().map(|&y| Point(*x1, y)).collect()
            }
            Segment(Point(m, y1), Point(n, y2)) if y1 == y2 => {
                let xrange = if m < n {
                    (*m..=*n).collect::<Vec<i32>>()
                } else {
                    (*n..=*m).rev().collect::<Vec<i32>>()
                };

                xrange.iter().map(|&x| Point(x, *y1)).collect()
            }
            Segment(Point(x1, y1), Point(x2, y2)) => {
                let xrange = if x1 < x2 {
                    (*x1..=*x2).collect::<Vec<i32>>()
                } else {
                    (*x2..=*x1).rev().collect::<Vec<i32>>()
                };

                let yrange = if y1 < y2 {
                    (*y1..=*y2).collect::<Vec<i32>>()
                } else {
                    (*y2..=*y1).rev().collect::<Vec<i32>>()
                };

                let pairwise = xrange.iter().zip(yrange.into_iter());
                pairwise.map(|(&x, y)| Point(x, y)).collect()
            }
        }
    }
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<Segment> {
    input
        .lines()
        // line = "1,2 -> 3,4"
        .map(|line| line.trim().replace(" -> ", ","))
        .map(|line| {
            // line = "1,2,3,4"
            line.split(",")
                .map(|d| d.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|line| {
            // line = [1, 2, 3, 4]
            let p1 = Point(line[0], line[1]);
            let p2 = Point(line[2], line[3]);

            Segment(p1, p2)
        })
        .collect::<Vec<Segment>>()
}

#[aoc(day5, part1, original)]
pub fn solver_part1(input: &[Segment]) -> usize {
    let mut map: HashMap<Point, i32> = HashMap::new();

    input
        .iter()
        .filter(|segment| !segment.is_diagonal())
        .for_each(|segment| {
            segment.get_points().iter().for_each(|p| {
                let map_point = map.entry(*p).or_insert(0);
                *map_point += 1;
            })
        });

    map.iter().filter(|(_, &i)| i >= 2).count()
}

#[aoc(day5, part2, original)]
pub fn solver_part2(input: &[Segment]) -> usize {
    let mut map: HashMap<Point, i32> = HashMap::new();

    input.iter().for_each(|segment| {
        segment.get_points().iter().for_each(|p| {
            let map_point = map.entry(*p).or_insert(0);
            *map_point += 1;
        })
    });

    map.iter().filter(|(_, &i)| i >= 2).count()
}

// :: ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input: &str = "0,9 -> 5,9
                           8,0 -> 0,8
                           9,4 -> 3,4
                           2,2 -> 2,1
                           7,0 -> 7,4
                           6,4 -> 2,0
                           0,9 -> 2,9
                           3,4 -> 1,4
                           0,0 -> 8,8
                           5,5 -> 8,2";

        assert_eq!(solver_part1(&generator(input)), 5);
    }

    #[test]
    fn example_2() {
        let input: &str = "0,9 -> 5,9
                           8,0 -> 0,8
                           9,4 -> 3,4
                           2,2 -> 2,1
                           7,0 -> 7,4
                           6,4 -> 2,0
                           0,9 -> 2,9
                           3,4 -> 1,4
                           0,0 -> 8,8
                           5,5 -> 8,2";

        assert_eq!(solver_part2(&generator(input)), 12);
    }
}
