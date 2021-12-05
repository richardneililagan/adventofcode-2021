use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point(i32, i32);

#[derive(Debug)]
pub struct Segment(Point, Point);

impl Segment {
    /// Get the points that are on this line segment, including the endpoints.
    fn get_points(&self) -> Option<Vec<Point>> {
        match self {
            Segment(Point(x1, m), Point(x2, n)) if x1 == x2 => {
                let miny = min(*m, *n) + 1;
                let maxy = max(*m, *n);
                let mut points: Vec<Point> = vec![self.0, self.1];

                (miny..maxy)
                    .map(|y| Point(*x1, y))
                    .for_each(|p| points.push(p));

                Some(points)
            }
            Segment(Point(m, y1), Point(n, y2)) if y1 == y2 => {
                let minx = min(*m, *n) + 1;
                let maxx = max(*m, *n);
                let mut points: Vec<Point> = vec![self.0, self.1];

                (minx..maxx)
                    .map(|x| Point(x, *y1))
                    .for_each(|p| points.push(p));

                Some(points)
            }
            _ => None,
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

    input.iter().for_each(|segment| {
        let result_points = segment.get_points();
        match result_points {
            None => (),
            Some(points) => points.iter().for_each(|p| {
                let map_point = map.entry(*p).or_insert(0);
                *map_point += 1;
            }),
        }
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
}
