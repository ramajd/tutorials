use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let cmd = split.next().ok_or("invalid command")?;
        match cmd {
            "rect" => {
                // rect AxB
                let dimensions = split.next().ok_or("invalid command")?;
                let mut split = dimensions.split('x');
                let col: usize = split.next().and_then(|c| c.parse().ok()).ok_or("invalid command")?;
                let row: usize = split.next().and_then(|r| r.parse().ok()).ok_or("invalid command")?;
                Ok(Command::Rect(col, row))
            }
            "rotate" => {
                // rotate row y=A by B
                // rotate column x=A by B
                let direction = split.next().ok_or("invalid command")?.trim();
                let index: usize = split
                    .next()
                    .and_then(|i| i.replace("y=", "").replace("x=", "").parse().ok())
                    .ok_or("invalid command")?;
                let count: usize = split.nth(1).and_then(|c| c.parse().ok()).ok_or("invalid command")?;
                match direction {
                    "row" => Ok(Command::RotateRow(index, count)),
                    "column" => Ok(Command::RotateCol(index, count)),
                    _ => Err("invalid command"),
                }
            }
            _ => Err("invalid command"),
        }
    }
}

#[derive(Debug)]
struct Screen {
    pixels: Vec<Vec<bool>>,
    cols: usize,
    rows: usize,
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut screen = String::new();
        for r in 0..self.rows {
            let mut row = String::new();
            for c in 0..self.cols {
                let status = if self.pixels[c][r] == true { "*" } else { "_" };
                row += format!("{} ", status).as_str();
            }
            screen += format!("{}\n", row).as_str();
        }

        write!(f, "{}\n", screen)
    }
}

impl Screen {
    fn new(cols: usize, rows: usize) -> Self {
        Self {
            pixels: vec![vec![false; rows]; cols],
            cols,
            rows,
        }
    }

    fn count_pixels(&self) -> usize {
        self.pixels
            .iter()
            .map(|c| c.iter().filter(|c| **c == true).count())
            .sum()
    }

    fn show(&mut self, commands: &Vec<Command>) {
        for command in commands {
            match command {
                Command::Rect(c, r) => {
                    let c = c % self.cols;
                    let r = r % self.rows;
                    for i in 0..c {
                        for j in 0..r {
                            self.pixels[i][j] = true;
                        }
                    }
                }
                Command::RotateRow(row_idx, count) => {
                    let row_idx = row_idx % self.rows;
                    let mut new_row = vec![false; self.cols];
                    for i in 0..self.cols {
                        let new_position = (i + count) % self.cols;
                        new_row[new_position] = self.pixels[i][row_idx];
                    }
                    for (idx, c) in new_row.iter().enumerate() {
                        self.pixels[idx][row_idx] = *c;
                    }
                }
                Command::RotateCol(col_idx, count) => {
                    let col_idx = col_idx % self.cols;
                    let mut new_col = vec![false; self.rows];
                    for i in 0..self.rows {
                        let new_position = (i + count) % self.rows;
                        new_col[new_position] = self.pixels[col_idx][i];
                    }
                    for (idx, c) in new_col.iter().enumerate() {
                        self.pixels[col_idx][idx] = *c;
                    }
                }
            }
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buffer = BufReader::new(file);

    let mut commands = Vec::new();

    for line in buffer.lines() {
        let cmd: Command = line.unwrap().parse().unwrap();
        commands.push(cmd);
    }

    let mut screen = Screen::new(50, 6);
    screen.show(&commands);
    println!("{}", screen);

    println!("number of lit pixels = {}", screen.count_pixels());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!("rect 3x2".parse(), Ok(Command::Rect(3, 2)));
        assert_eq!("rotate column x=1 by 1".parse(), Ok(Command::RotateCol(1, 1)));
        assert_eq!("rotate row y=0 by 4".parse(), Ok(Command::RotateRow(0, 4)));
    }

    #[test]
    fn test_rect() {
        let mut screen = Screen::new(7, 3);
        screen.show(&vec![Command::Rect(3, 2)]);
        println!("{}", screen);
        assert_eq!(screen.count_pixels(), 6);
        assert!(screen.pixels[0][0]);
        assert!(screen.pixels[0][1]);
        assert!(screen.pixels[1][0]);
        assert!(screen.pixels[1][1]);
        assert!(screen.pixels[2][0]);
        assert!(screen.pixels[2][1]);
    }

    #[test]
    fn test_rotate() {
        let mut screen = Screen::new(7, 3);
        screen.show(&vec![Command::Rect(3, 2), Command::RotateCol(1, 1)]);
        println!("{}", screen);
        assert!(!screen.pixels[1][0]);
        assert!(screen.pixels[1][2]);
        screen.show(&vec![Command::RotateRow(0, 4)]);
        println!("{}", screen);
        assert!(screen.pixels[4][0]);
        assert!(screen.pixels[6][0]);
        assert!(!screen.pixels[0][0]);
        assert!(!screen.pixels[2][0]);
        screen.show(&vec![Command::RotateCol(1, 1)]);
        println!("{}", screen);
        assert!(screen.pixels[1][0]);
        assert!(!screen.pixels[1][1]);
    }
}
