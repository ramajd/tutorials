use std::{collections::HashMap, fmt::Display, vec};

pub type Result<T> = std::result::Result<T, KitError>;

#[derive(Debug, Clone)]
pub struct KitError {
    message: String,
}

impl Display for KitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[KitError]: {}", self.message)
    }
}

impl KitError {
    fn new(msg: &str) -> Self {
        Self {
            message: msg.to_owned(),
        }
    }
}

#[derive(Debug)]
enum Operator {
    ASSIGN,
    NOT,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
}

#[derive(Debug)]
struct Wire {
    title: String,
    operator: Operator,
    inputs: Vec<String>,
}

impl Wire {
    fn parse(instruction: &str) -> Result<Self> {
        let mut split = instruction.trim().split("->");
        let inputs = split.next();
        let output = split.next();
        if inputs.is_none() || output.is_none() {
            return Err(KitError::new("Failed to parse instruction"));
        }
        let input_parts: Vec<String> = inputs
            .unwrap()
            .trim()
            .split(" ")
            .map(|p| p.trim().to_owned())
            .collect();
        let operator = match input_parts.len() {
            1 => Operator::ASSIGN,
            2 => match input_parts[0].as_ref() {
                "NOT" => Operator::NOT,
                _ => return Err(KitError::new("Unknown operator")),
            },
            3 => match input_parts[1].as_ref() {
                "AND" => Operator::AND,
                "OR" => Operator::OR,
                "LSHIFT" => Operator::LSHIFT,
                "RSHIFT" => Operator::RSHIFT,
                _ => return Err(KitError::new("Unknown operator")),
            },
            _ => return Err(KitError::new("Invalid instruction")),
        };
        let inputs = match operator {
            Operator::ASSIGN => vec![input_parts[0].clone()],
            Operator::NOT => vec![input_parts[1].clone()],
            Operator::AND | Operator::OR | Operator::LSHIFT | Operator::RSHIFT => {
                vec![input_parts[0].clone(), input_parts[2].clone()]
            }
        };
        Ok(Wire {
            title: output.unwrap().trim().to_owned(),
            operator,
            inputs,
        })
    }
}

pub struct Kit {
    wires: Vec<Wire>,
    values: HashMap<String, u16>,
}

impl Kit {
    pub fn new() -> Self {
        Self {
            wires: Vec::new(),
            values: HashMap::new(),
        }
    }

    pub fn get_signal(&self, name: &str) -> Result<u16> {
        self.values
            .get(name)
            .cloned()
            .ok_or(KitError::new("Signal not found"))
    }

    pub fn add_instruction(&mut self, instruction: &str) {
        let wire = Wire::parse(instruction).unwrap();
        let value = self.process_wire(&wire);

        if let Some(value) = value {
            println!("{:?}\t -> {:?}", wire, value);
            let key = wire.title.clone();
            self.update_wires(&key, value);
        }
        self.wires.push(wire);
    }

    fn process_wire(&self, wire: &Wire) -> Option<u16> {
        let get_value = |key: &str| -> Option<u16> {
            key.parse::<u16>()
                .ok()
                .or_else(|| self.values.get(key).cloned())
        };

        if let Some(value) = self.values.get(&wire.title) {
            return Some(*value);
        }

        // println!("{:?}", wire);
        match wire.operator {
            Operator::ASSIGN => get_value(&wire.inputs[0]),
            Operator::NOT => get_value(&wire.inputs[0]).and_then(|v| Some(!v)),
            Operator::LSHIFT => get_value(&wire.inputs[0])
                .and_then(|v| Some(v << &wire.inputs[1].parse::<u16>().unwrap())),
            Operator::RSHIFT => get_value(&wire.inputs[0])
                .and_then(|v| Some(v >> &wire.inputs[1].parse::<u16>().unwrap())),
            Operator::AND => {
                let v1 = get_value(&wire.inputs[0]);
                let v2 = get_value(&wire.inputs[1]);
                if let (Some(v1), Some(v2)) = (v1, v2) {
                    Some(v1 & v2)
                } else {
                    None
                }
            }
            Operator::OR => {
                let v1 = get_value(&wire.inputs[0]);
                let v2 = get_value(&wire.inputs[1]);
                if let (Some(v1), Some(v2)) = (v1, v2) {
                    Some(v1 | v2)
                } else {
                    None
                }
            }
        }
    }

    fn update_wires(&mut self, key: &String, value: u16) {
        self.values.insert(key.clone(), value);

        let mut modifications: HashMap<String, u16> = HashMap::new();

        for wire in self.wires.iter().filter(|w| w.inputs.contains(key)) {
            if let Some(value) = self.process_wire(wire) {
                println!("{:?}\t -> {:?}", wire, value);
                modifications.insert(wire.title.clone(), value);
            }
        }
        for (k, v) in modifications {
            self.update_wires(&k, v);
        }
    }
}
