use itertools::Itertools;
use num::rational::Ratio;
use std::collections::{HashSet, VecDeque};
use std::convert::TryFrom;
use std::error::Error;
use std::ops::{Add, Sub};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    fn straight_line_to(&self, target: &Point) -> StraightLinePointsIter {
        StraightLinePointsIter {
            current: *self,
            target: *target,
        }
    }
    fn has_line_of_sight_to(&self, target: &Point, obstructions: &HashSet<Point>) -> bool {
        !self
            .straight_line_to(target)
            .any(|p| obstructions.contains(&p))
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("day10/input.txt")?;

    let mut asteroids: HashSet<Point> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, value) in line.chars().enumerate() {
            if value == '#' {
                let x = i32::try_from(x)?;
                let y = i32::try_from(y)?;
                let Point = Point { x, y };
                asteroids.insert(Point);
            }
        }
    }

    let asteroids: HashSet<Point> = asteroids;
    let (part1_ans, &station_point) = asteroids
        .iter()
        .map(|asteroid| {
            let in_sight = asteroids
                .iter()
                .filter(|&x| x != asteroid)
                .filter(|x| asteroid.has_line_of_sight_to(x, &asteroids))
                .count();
            (in_sight, asteroid)
        })
        .max_by(|(in_sight_1, _), (in_sight_2, _)| in_sight_1.cmp(in_sight_2))
        .unwrap();

    println!("{:?}", part1_ans);
    println!("{:?}", station_point);

    let mut asteroids = asteroids;
    asteroids.remove(&station_point);
    let mut asteroids: Vec<Point> = asteroids.iter().copied().collect::<Vec<_>>();
    asteroids.sort_unstable_by(|a, b| clockwise_ordering(station_point, a, b));

    let mut clockwise_groups = asteroids
        .iter()
        .copied()
        .map(|p| p - station_point)
        .group_by(|p: &Point| match (p.x, p.y) {
            (0, 0) => (0, 0),
            (x, 0) => (x.signum(), 0),
            (x, y) => {
                let ratio = Ratio::new(x, y);
                let x = ratio.numer().abs() * x.signum();
                let y = ratio.denom().abs() * y.signum();
                (x, y)
            }
        })
        .into_iter()
        .map(|(key, group)| {
            let mut group: Vec<Point> = group.collect();
            group.sort_unstable_by(|a, b| (a.x * a.x + a.y * a.y).cmp(&(b.x * b.x + b.y * b.y)));
            group.into_iter().collect()
        })
        .collect::<Vec<VecDeque<Point>>>();

    let mut destroyed_asteroids = 0;
    let mut asteroid_200: Option<Point> = None;
    for group in clockwise_groups.iter_mut() {
        if let Some(asteroid) = group.pop_front() {
            destroyed_asteroids += 1;
            if destroyed_asteroids == 200 {
                asteroid_200.replace(asteroid);
                println!("asteroid 200: {:?}", asteroid);
            }
        }
    }

    let part2_ans = {
        let p = asteroid_200.unwrap() + station_point;
        p.x * 100 + p.y
    };

    println!("{}", part2_ans);

    Ok(())
}

fn clockwise_ordering(center: Point, a: &Point, b: &Point) -> std::cmp::Ordering {
    let ref a = *a - center;
    let ref b = *b - center;

    clockwise_angle(a).partial_cmp(&clockwise_angle(b)).unwrap()
}
fn clockwise_angle(p: &Point) -> f64 {
    let x = f64::from(p.x);
    let y = f64::from(-p.y);

    let two_pi = 2.0 * std::f64::consts::PI;

    (x.atan2(y) + two_pi) % two_pi
}

struct StraightLinePointsIter {
    current: Point,
    target: Point,
}
impl Iterator for StraightLinePointsIter {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.target {
            return None;
        }

        let x_dist = self.target.x - self.current.x;
        let y_dist = self.target.y - self.current.y;

        if y_dist == 0 {
            let x_step = x_dist.signum();
            self.current.x += x_step;
        } else if x_dist == 0 {
            let y_step = y_dist.signum();
            self.current.y += y_step;
        } else {
            let ratio = Ratio::new(x_dist, y_dist);

            let x_step = ratio.numer();
            let y_step = ratio.denom();

            self.current.x += x_step.abs() * x_dist.signum();
            self.current.y += y_step.abs() * y_dist.signum();
        }

        if self.current == self.target {
            None
        } else {
            Some(self.current)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clockwise_ordering() {
        let center = Point { x: 0, y: 0 };
        let mut points = vec![
            Point { x: 0, y: -1 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: -1, y: 0 },
            Point { x: -1, y: 1 },
            Point { x: -1, y: -1 },
            Point { x: 1, y: -1 },
        ];

        points.sort_unstable_by(|a, b| clockwise_ordering(center, a, b));

        let sorted = vec![
            Point { x: 0, y: -1 },
            Point { x: 1, y: -1 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 0, y: 1 },
            Point { x: -1, y: 1 },
            Point { x: -1, y: 0 },
            Point { x: -1, y: -1 },
        ];

        assert_eq!(points, sorted);
    }
}
