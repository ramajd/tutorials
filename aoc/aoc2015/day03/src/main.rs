use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;

struct Board {
    x: isize,
    y: isize,
    visit_map: HashMap<String, usize>,
}

impl Board {
    fn new() -> Self {
        let visit_map = HashMap::new();
        let mut result = Self {
            x: 0,
            y: 0,
            visit_map,
        };
        result.mark_visited();
        result
    }

    fn mark_visited(&mut self) {
        let key = format!("{}_{}", self.x, self.y);
        self.visit_map
            .entry(key)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    fn update_position(&mut self, direction: char) -> Result<(), &str> {
        match direction {
            '>' => self.x += 1,
            '<' => self.x -= 1,
            'v' => self.y += 1,
            '^' => self.y -= 1,
            _ => {
                return Err("Invalid Transition");
            }
        };
        Ok(())
    }

    fn perform_transition(&mut self, direction: char) {
        if let Ok(_) = self.update_position(direction) {
            self.mark_visited()
        }
    }

    fn visit_count(&self) -> usize {
        self.visit_map.keys().len()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, {}] - Visit count: {}",
            self.x,
            self.y,
            self.visit_count()
        )
    }
}

fn main() {
    let mut file = File::open("input.txt").expect("Failed to open input.txt");
    let mut input = String::new();

    file.read_to_string(&mut input)
        .expect("Failed to read input");

    let mut board = Board::new();

    println!("Board: {}", board);

    for c in input.chars() {
        board.perform_transition(c);
    }
    println!("Board: {}", board);
}
