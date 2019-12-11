use std::error::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl std::convert::From<Direction> for Coord {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Coord { x: 0, y: 1 },
            Direction::Down => Coord { x: 0, y: -1 },
            Direction::Left => Coord { x: -1, y: 0 },
            Direction::Right => Coord { x: 1, y: 0 },
        }
    }
}

impl std::ops::Mul<i32> for Coord {
    type Output = Coord;
    fn mul(self, rhs: i32) -> Self::Output {
        Coord {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::str::FromStr for Direction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err("invalid direction"),
        }
    }
}

trait IsBetween<T> {
    fn is_between(self, a: T, b: T) -> bool;
}
impl IsBetween<i32> for i32 {
    fn is_between(self, a: i32, b: i32) -> bool {
        (a < self && self < b) || (a > self && self > b)
    }
}

struct Wire(Vec<Line>);
impl Wire {
    pub fn coords_iter(&self) -> impl Iterator<Item = Coord> + '_ {
        let starting_coord_iter = self
            .0
            .first()
            .into_iter()
            .flat_map(|line| line.coords_iter());

        starting_coord_iter.chain(self.0.iter().flat_map(|line| line.coords_iter()))
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Coord,
    direction: Direction,
    distance: u16,
}
impl Line {
    pub fn coords_iter(&self) -> impl Iterator<Item = Coord> {
        let to_add = Coord::from(self.direction);
        let mut coord = self.start;

        std::iter::repeat_with(move || {
            coord = coord + to_add;
            coord
        })
        .take(usize::from(self.distance))
    }

    pub fn end(&self) -> Coord {
        self.start + Coord::from(self.direction) * i32::from(self.distance)
    }

    fn new(start: Coord, direction: Direction, distance: u16) -> Self {
        Line {
            start,
            direction,
            distance,
        }
    }
    fn intersection(&self, other: &Line) -> Option<Coord> {
        if self.is_vertical()
            && other.is_horizontal()
            && self.start.x.is_between(other.start.x, other.end().x)
            && other.start.y.is_between(self.start.y, self.end().y)
        {
            Some(Coord {
                x: self.start.x,
                y: other.start.y,
            })
        } else if self.is_horizontal()
            && other.is_vertical()
            && self.start.y.is_between(other.start.y, other.end().y)
            && other.start.x.is_between(self.start.x, self.end().x)
        {
            Some(Coord {
                x: other.start.x,
                y: self.start.y,
            })
        } else {
            None
        }
    }

    fn is_horizontal(&self) -> bool {
        match self.direction {
            Direction::Left | Direction::Right => true,
            Direction::Up | Direction::Down => false,
        }
    }
    fn is_vertical(&self) -> bool {
        !self.is_horizontal()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("day03/input.txt")?;
    let mut input_lines = input.lines();

    let wire1: &str = input_lines.next().unwrap();
    let wire1_lines: Vec<&str> = wire1.split(",").collect();
    let wire1_lines = wire1_lines
        .iter()
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            let direction: Direction = direction.parse().unwrap();
            let distance: u16 = distance.parse().unwrap();
            (direction, distance)
        })
        .scan(Coord { x: 0, y: 0 }, |from_point, (dir, dist)| {
            let line = Line::new(*from_point, dir, dist);
            *from_point = line.end();
            Some(line)
        })
        .collect::<Vec<_>>();

    let wire2: &str = input_lines.next().unwrap();
    let wire2_lines: Vec<&str> = wire2.split(",").collect();
    let wire2_lines = wire2_lines
        .iter()
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            let direction: Direction = direction.parse().unwrap();
            let distance: u16 = distance.parse().unwrap();
            (direction, distance)
        })
        .scan(Coord { x: 0, y: 0 }, |from_point, (dir, dist)| {
            let line = Line::new(*from_point, dir, dist);
            *from_point = line.end();
            Some(line)
        })
        .collect::<Vec<_>>();

    let mut intersections = std::collections::HashSet::new();
    for line1 in wire1_lines.clone() {
        for line2 in &wire2_lines {
            if let Some(coord) = line1.intersection(line2) {
                intersections.insert(coord);
            }
        }
    }

    let closest = intersections
        .iter()
        .min_by(|a, b| (a.x.abs() + a.y.abs()).cmp(&(b.x.abs() + b.y.abs())))
        .unwrap();

    println!("part1 ans: {}", closest.x.abs() + closest.y.abs());

    use std::collections::HashMap;
    let mut wire1_steps: HashMap<Coord, usize> = HashMap::new();
    for (steps, coords) in Wire(wire1_lines.clone()).coords_iter().enumerate() {
        if intersections.contains(&coords) {
            wire1_steps.insert(coords, steps);
        }
    }
    let mut wire2_steps: HashMap<Coord, usize> = HashMap::new();
    for (steps, coords) in Wire(wire2_lines.clone()).coords_iter().enumerate() {
        if intersections.contains(&coords) {
            wire2_steps.insert(coords, steps);
        }
    }

    let mut values = vec![];
    for (key, value) in wire1_steps {
        if let Some(other_value) = wire2_steps.get(&key) {
            values.push(value + other_value)
        }
    }

    println!("part2 ans: {}", values.iter().min().unwrap());

    Ok(())
}
