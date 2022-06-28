use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::usize;

type GridRow = Vec<bool>;

#[derive(Clone, Debug, PartialEq)]
struct Grid {
    rows: Vec<GridRow>,
}
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for (r, row) in self.rows.iter().enumerate() {
            result += &format!("[{}]: ", r);
            for cell in row {
                result += &format!("{} ", if *cell == true { '#' } else { '.' });
            }
            result += "\n";
        }
        write!(f, "{}", result)
    }
}

impl Grid {
    fn count_lights(&self) -> usize {
        let mut result = 0;
        for row in &self.rows {
            for cell in row {
                if *cell == true {
                    result += 1;
                }
            }
        }
        result
    }

    pub fn iterate(&mut self) {
        let mut new_state = Vec::new();
        let row_count = self.rows.len();
        for r in 0..row_count {
            let col_count = self.rows[r].len();
            let mut new_row = Vec::new();
            for c in 0..col_count {
                let new_status = self.calculate_status(r, c);
                new_row.push(new_status);
            }
            new_state.push(new_row);
        }
        self.rows = new_state;
    }

    pub fn calculate_status(&self, r: usize, c: usize) -> bool {
        let current_status = self.rows[r][c];
        let neighbors = self.count_neighbors(r, c);
        match (current_status, neighbors) {
            (true, 2 | 3) => true,
            (true, _) => false,
            (false, 3) => true,
            (false, _) => false,
        }
    }

    fn count_neighbors(&self, r: usize, c: usize) -> usize {
        let mut count = 0;
        let row_count = self.rows.len();
        let r_start = if r > 0 { r - 1 } else { r };
        let r_end = if r == row_count - 1 { r } else { r + 1 };
        let col_count = self.rows[r].len();
        let c_start = if c > 0 { c - 1 } else { c };
        let c_end = if c == col_count - 1 { c } else { c + 1 };

        // println!("[{},{}]: from {},{} to {},{}",r, c, r_start, c_start, r_end, c_end);
        for rr in r_start..=r_end {
            for cc in c_start..=c_end {
                if rr != r || cc != c {
                    if self.rows[rr][cc] == true {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    pub fn iterate_with_stuck_lights(&mut self) {
        let mut new_state = Vec::new();
        let row_count = self.rows.len();
        let col_count = self.rows[0].len();
        for r in 0..row_count {
            let mut new_row = Vec::new();
            for c in 0..col_count {
                let new_status = self.calculate_status(r, c);
                new_row.push(new_status);
            }
            new_state.push(new_row);
        }
        new_state[0][0] = true;
        new_state[0][col_count - 1] = true;
        new_state[row_count - 1][0] = true;
        new_state[row_count - 1][col_count - 1] = true;
        self.rows = new_state;
    }
}

fn main() {
    let mut file = File::open("input.txt").expect("Failed to open input.txt");
    let mut raw_data = String::new();
    file.read_to_string(&mut raw_data)
        .expect("Failed to read input");

    let steps = 100;

    let mut grid = parse_grid(raw_data).expect("Failed to parse Grid");
    let mut second_grid = grid.clone();

    for _ in 0..steps {
        grid.iterate();
    }
    println!(
        "Number of lights after {} iterations equal to {}",
        steps,
        grid.count_lights()
    );

    second_grid.rows[0][0] = true;
    second_grid.rows[0][99] = true;
    second_grid.rows[99][0] = true;
    second_grid.rows[99][99] = true;

    for _ in 0..steps {
        second_grid.iterate_with_stuck_lights();
    }
    println!(
        "Number of lights (with stock ones) after {} iterations equal to {}",
        steps,
        second_grid.count_lights()
    );
}

fn parse_grid(raw_data: String) -> Result<Grid, std::fmt::Error> {
    let mut grid = Grid { rows: Vec::new() };
    for line in raw_data.lines() {
        let row = parse_row(line)?;
        grid.rows.push(row);
    }
    Ok(grid)
}

fn parse_row(line: &str) -> Result<GridRow, std::fmt::Error> {
    let mut row = GridRow::new();
    for c in line.trim().chars() {
        let status = match c {
            '#' => true,
            '.' => false,
            _ => return Err(std::fmt::Error),
        };
        row.push(status);
    }
    Ok(row)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_parser() {
        let input = ".#.#.#";
        assert_eq!(
            parse_row(input),
            Ok(vec![false, true, false, true, false, true])
        );
        let input = ".#.#.#$";
        assert_eq!(parse_row(input), Err(std::fmt::Error));
    }

    #[test]
    fn test_parser() {
        let initial_state = String::from(
            r#"##.#.#
            ...##.
            #....#
            ..#...
            #.#..#
            ####.#"#,
        );
        let grid = parse_grid(initial_state);

        let created = Grid {
            rows: vec![
                vec![true, true, false, true, false, true],    // ##.#.#
                vec![false, false, false, true, true, false],  // ...##.
                vec![true, false, false, false, false, true],  // #....#
                vec![false, false, true, false, false, false], // ..#...
                vec![true, false, true, false, false, true],   // #.#..#
                vec![true, true, true, true, false, true],     // ####.#
            ],
        };
        assert_eq!(grid, Ok(created));
    }

    #[test]
    fn test_count_neighbors() {
        let grid = Grid {
            rows: vec![
                vec![false, true, false],
                vec![true, false, true],
                vec![false, true, false],
            ],
        };
        assert!(grid.count_neighbors(0, 0) == 2);
        assert!(grid.count_neighbors(0, 1) == 2);
        assert!(grid.count_neighbors(0, 2) == 2);
        assert!(grid.count_neighbors(1, 0) == 2);
        assert!(grid.count_neighbors(1, 1) == 4);
        assert!(grid.count_neighbors(1, 2) == 2);
        assert!(grid.count_neighbors(2, 0) == 2);
        assert!(grid.count_neighbors(2, 1) == 2);
        assert!(grid.count_neighbors(2, 2) == 2);
    }

    #[test]
    fn test_calculate_status() {
        let grid = Grid {
            rows: vec![
                vec![false, true, false, true],
                vec![true, false, true, false],
                vec![false, true, true, false],
                vec![false, true, false, false],
            ],
        };

        assert!(grid.calculate_status(0, 0) == false);
        assert!(grid.calculate_status(0, 1) == true);
        assert!(grid.calculate_status(2, 2) == true);
        assert!(grid.calculate_status(2, 0) == true);
        assert!(grid.calculate_status(2, 0) == true);
        assert!(grid.calculate_status(0, 3) == false);
    }

    #[test]
    fn test_iteration() {
        let mut grid = parse_grid(String::from(
            r#".#.#.#
               ...##.
               #....#
               ..#...
               #.#..#
               ####.."#,
        ))
        .unwrap();
        println!("{}", grid);

        grid.iterate();

        let next = parse_grid(String::from(
            r#"..##..
               ..##.#
               ...##.
               ......
               #.....
               #.##.."#,
        ))
        .unwrap();
        assert_eq!(grid, next);
    }

    #[test]
    fn test_iteration_with_stuck_lights() {
        let mut grid = parse_grid(String::from(
            r#"##.#.#
               ...##.
               #....#
               ..#...
               #.#..#
               ####.#"#,
        ))
        .unwrap();
        println!("{}", grid);

        grid.iterate_with_stuck_lights();

        let next = parse_grid(String::from(
            r#"#.##.#
            ####.#
            ...##.
            ......
            #...#.
            #.####"#,
        ))
        .unwrap();
        assert_eq!(grid, next);
    }
}
