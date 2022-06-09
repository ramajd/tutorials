use std::fs::File;
use std::io::{prelude::*, BufReader};

struct GiftBox {
    dimensions: [usize; 3],
}

impl GiftBox {
    pub fn parse(line: &String) -> Option<Self> {
        let converted: Vec<usize> = line
            .split("x")
            .map(|f| f.parse::<usize>())
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();
        if converted.len() == 3 {
            return Some(GiftBox {
                dimensions: converted.try_into().unwrap(),
            });
        }
        None
    }

    fn surface(&self) -> usize {
        let s1 = self.dimensions[0] * self.dimensions[1];
        let s2 = self.dimensions[0] * self.dimensions[2];
        let s3 = self.dimensions[1] * self.dimensions[2];
        return 2 * s1 + 2 * s2 + 2 * s3;
    }

    fn min_dimensions(&self) -> (usize, usize) {
        let max = self.dimensions.into_iter().max().unwrap();

        let result = if self.dimensions[0] == max {
            (self.dimensions[1], self.dimensions[2])
        } else if self.dimensions[1] == max {
            (self.dimensions[0], self.dimensions[2])
        } else {
            /* self.dimensions[2] == max */
            (self.dimensions[0], self.dimensions[1])
        };
        result
    }

    fn slack(&self) -> usize {
        let dims = self.min_dimensions();
        return dims.0 * dims.1;
    }

    fn volume(&self) -> usize {
        self.dimensions[0] * self.dimensions[1] * self.dimensions[2]
    }

    fn shortest_perimeter(&self) -> usize {
        let dims = self.min_dimensions();
        return 2 * (dims.0 + dims.1);
    }

    pub fn paper_size(&self) -> usize {
        return self.surface() + self.slack();
    }

    pub fn ribbon_length(&self) -> usize {
        return self.shortest_perimeter() + self.volume();
    }
}

fn main() {
    let file = File::open("input.txt").expect("Failed to read input");
    let buffer = BufReader::new(file);

    let mut total_papaer = 0;
    let mut total_ribbon = 0;

    for line in buffer.lines() {
        if let Ok(line) = line {
            if let Some(gift) = GiftBox::parse(&line) {
                total_papaer += gift.paper_size();
                total_ribbon += gift.ribbon_length();
            }
        }
    }

    println!("total paper surface: {}", total_papaer);
    println!("total ribbon length: {}", total_ribbon);
}
