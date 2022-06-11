use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum BoardError {
    ErrorStr(String),
}

pub type Result<T> = std::result::Result<T, BoardError>;

impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardError::ErrorStr(msg) => write!(f, "Board Error: {}", msg),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

pub struct Worker {
    title: String,
    position: Position,
}

impl Worker {
    fn transit(&mut self, direction: char) -> Result<Position> {
        match direction {
            '>' => self.position.x += 1,
            '<' => self.position.x -= 1,
            'v' => self.position.y += 1,
            '^' => self.position.y -= 1,
            _ => {
                return Err(BoardError::ErrorStr("Invalid direction".to_owned()));
            }
        }
        Ok(self.position.clone())
    }
}

impl Display for Worker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]: ({}, {})",
            self.title, self.position.x, self.position.y
        )
    }
}

pub struct Board {
    workers: Vec<Worker>,
    visit_map: HashMap<Position, usize>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            workers: Vec::new(),
            visit_map: HashMap::new(),
        }
    }

    pub fn add_worker(&mut self, title: &str) {
        let worker = Worker {
            title: title.to_owned(),
            position: Position { x: 0, y: 0 },
        };

        let worker_position = worker.position.clone();

        self.workers.push(worker);
        self.mark_visited(worker_position);
    }

    pub fn visit_count(&self) -> usize {
        self.visit_map.keys().len()
    }

    pub fn perform_transitions(&mut self, transitions: String) {
        for (idx, direction) in transitions.chars().enumerate() {
            let worker_index = idx % self.workers.len();
            self.perform_transition(worker_index, direction);
        }
    }

    fn perform_transition(&mut self, worker_index: usize, direction: char) {
        if let Ok(pos) = self.update_worker_position(worker_index, direction) {
            self.mark_visited(pos);
        }
    }

    fn update_worker_position(&mut self, worker_index: usize, direction: char) -> Result<Position> {
        let worker = self.workers.get_mut(worker_index).unwrap();
        worker.transit(direction)
    }

    fn mark_visited(&mut self, position: Position) {
        self.visit_map
            .entry(position)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
}
