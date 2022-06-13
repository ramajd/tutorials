use std::{fmt::Display, ops::Add};

#[derive(Debug, Clone)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub enum Command {
    TurnOn(Coordinate, Coordinate),
    TurnOff(Coordinate, Coordinate),
    Toggle(Coordinate, Coordinate),
}

#[derive(Debug, Clone)]
pub struct GridCommandError;

impl Display for GridCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to Apply command on grid")
    }
}

#[derive(Debug)]
pub struct Grid {
    cells: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        Self {
            cells: vec![vec![false; size]; size],
        }
    }

    fn adjust_coordinates(
        &self,
        c1: &Coordinate,
        c2: &Coordinate,
    ) -> Result<(Coordinate, Coordinate), GridCommandError> {
        if c1.x < self.cells.len()
            && c2.x < self.cells.len()
            && c1.y < self.cells[0].len()
            && c2.y < self.cells[0].len()
        {
            let x_start = if c1.x < c2.x { c1.x } else { c2.x };
            let x_end = if c1.x > c2.x { c1.x } else { c2.x };
            let y_start = if c1.y < c2.y { c1.y } else { c2.y };
            let y_end = if c1.y > c2.y { c1.y } else { c2.y };
            Ok((
                Coordinate {
                    x: x_start,
                    y: y_start,
                },
                Coordinate { x: x_end, y: y_end },
            ))
        } else {
            Err(GridCommandError)
        }
    }

    pub fn apply_command(&mut self, command: Command) -> Result<(), GridCommandError> {
        match command {
            Command::TurnOn(c1, c2) => {
                let (start, end) = self.adjust_coordinates(&c1, &c2)?;
                for x in start.x..=end.x {
                    for y in start.y..=end.y {
                        self.cells[x][y] = true;
                    }
                }
            }
            Command::TurnOff(c1, c2) => {
                let (start, end) = self.adjust_coordinates(&c1, &c2)?;
                for x in start.x..=end.x {
                    for y in start.y..=end.y {
                        self.cells[x][y] = false;
                    }
                }
            }
            Command::Toggle(c1, c2) => {
                let (start, end) = self.adjust_coordinates(&c1, &c2)?;
                for x in start.x..=end.x {
                    for y in start.y..=end.y {
                        self.cells[x][y] = !self.cells[x][y];
                    }
                }
            }
        }
        Ok(())
    }

    pub fn lit_count(&self) -> usize {
        self.cells.iter().fold(0, |acc, row| {
            acc + row
                .iter()
                .fold(0, |acc, cell| if *cell { acc + 1 } else { acc })
        })
    }
}

#[derive(Debug, Clone)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error parsing input")
    }
}

fn parse_coordinate(input: &str) -> Result<Coordinate, ParseError> {
    let parts = input.split(",").map(|p| p.trim()).collect::<Vec<&str>>();
    if parts.len() != 2 {
        return Err(ParseError);
    }
    Ok(Coordinate {
        x: parts[0].parse::<usize>().or(Err(ParseError))?,
        y: parts[1].parse::<usize>().or(Err(ParseError))?,
    })
}

pub fn extract_command(command: &str) -> Result<Command, ParseError> {
    // turn on 353,910 through 832,961
    // turn off 866,97 through 924,784
    // toggle 836,599 through 857,767
    let mut split = command.split(" ");
    let mut cmd = split.next().unwrap().to_owned();
    if cmd == "turn" {
        cmd = cmd.add(" ").add(split.next().unwrap());
    }
    let from = parse_coordinate(split.next().unwrap())?;
    split.next().unwrap(); // through
    let to = parse_coordinate(split.next().unwrap())?;

    match cmd.as_str() {
        "turn on" => Ok(Command::TurnOn(from, to)),
        "turn off" => Ok(Command::TurnOff(from, to)),
        "toggle" => Ok(Command::Toggle(from, to)),
        _ => Err(ParseError),
    }
}
