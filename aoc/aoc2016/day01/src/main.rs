use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Step {
    Left(isize),
    Right(isize),
}

impl FromStr for Step {
    type Err = String;

    fn from_str(step: &str) -> Result<Self, Self::Err> {
        let step = step.trim();
        let distance: isize = step[1..].parse().map_err(|_| format!("Invalid step: {}", step))?;
        match &step[..1] {
            "L" => Ok(Step::Left(distance)),
            "R" => Ok(Step::Right(distance)),
            _ => Err(format!("Invalid step: {}", step)),
        }
    }
}

impl Step {
    fn distance(&self) -> isize {
        match self {
            Step::Left(d) | Step::Right(d) => *d,
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
struct Position {
    heading: Direction,
    x: isize,
    y: isize,
    path: HashSet<(isize, isize)>,
}

impl Position {
    fn new() -> Self {
        Self {
            heading: Direction::North,
            x: 0,
            y: 0,
            path: HashSet::new(),
        }
    }

    fn perform_step(&mut self, step: &Step, stop_on_visited: bool) -> bool {
        self.turn(step);
        for _ in 0..step.distance() {
            if self.walk(1) && stop_on_visited {
                return true;
            }
        }
        false
    }

    fn distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }

    fn turn(&mut self, step: &Step) {
        self.heading = match (step, &self.heading) {
            (Step::Left(_), Direction::North) => Direction::West,
            (Step::Left(_), Direction::South) => Direction::East,
            (Step::Left(_), Direction::East) => Direction::North,
            (Step::Left(_), Direction::West) => Direction::South,
            (Step::Right(_), Direction::North) => Direction::East,
            (Step::Right(_), Direction::South) => Direction::West,
            (Step::Right(_), Direction::East) => Direction::South,
            (Step::Right(_), Direction::West) => Direction::North,
        };
    }

    fn walk(&mut self, distance: isize) -> bool {
        match self.heading {
            Direction::North => self.y += distance,
            Direction::South => self.y -= distance,
            Direction::East => self.x += distance,
            Direction::West => self.x -= distance,
        };
        !self.path.insert((self.x, self.y))
    }
}

fn main() {
    let mut file = File::open("input.txt").expect("Failed to open input.txt");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read input");

    let mut steps = Vec::new();

    for part in data.split(", ") {
        let step: Step = part.parse().unwrap();
        steps.push(step);
    }

    let position = find_hq(&steps, false);
    println!("HQ distance = {}", position.distance());

    let position = find_hq(&steps, true);
    println!("HQ Distance (1st visited) = {}", position.distance());
}

fn find_hq(steps: &Vec<Step>, stop_on_visited: bool) -> Position {
    let mut position = Position::new();
    for step in steps {
        if position.perform_step(step, stop_on_visited) && stop_on_visited {
            break;
        }
    }

    position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!("L1".parse(), Ok(Step::Left(1)));
        assert_eq!("R1".parse(), Ok(Step::Right(1)));
        assert_eq!("R12".parse(), Ok(Step::Right(12)));
        assert_eq!("LL".parse::<Step>(), Err(String::from("Invalid step: LL")));
    }

    #[test]
    fn test_find_hq() {
        assert_eq!(find_hq(&vec![Step::Right(2), Step::Left(3)], false).distance(), 5);
        assert_eq!(
            find_hq(&vec![Step::Right(2), Step::Right(2), Step::Right(2)], false).distance(),
            2
        );
        assert_eq!(
            find_hq(
                &vec![Step::Right(5), Step::Left(5), Step::Right(5), Step::Right(3),],
                false
            )
            .distance(),
            12
        );
    }

    #[test]
    fn test_until_visited_twice() {
        assert_eq!(
            find_hq(
                &vec![Step::Right(8), Step::Right(4), Step::Right(4), Step::Right(8)],
                true
            )
            .distance(),
            4
        );
    }
}
