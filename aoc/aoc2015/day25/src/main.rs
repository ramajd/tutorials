const STEP: usize = 252533;
const MOD: usize = 33554393;

fn main() {
    let first: usize = 20151125;
    let row: usize = 2978;
    let col: usize = 3083;
    println!(
        "code located in ({},{}) = {}",
        row,
        col,
        calc_next(first, row, col)
    );
}

fn calc_next(start: usize, row: usize, col: usize) -> usize {
    let mut next = start;
    let mut r = 1;
    let mut c = 1;
    loop {
        if r == row && c == col {
            break;
        }
        if r == 1 {
            r = c + 1;
            c = 1;
        } else {
            r -= 1;
            c += 1;
        }
        next = (next * STEP) % MOD;
    }

    next
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_numbers() {
        let start = 20151125;

        assert_eq!(calc_next(start, 1, 1), start);
        assert_eq!(calc_next(start, 2, 1), 31916031);
        assert_eq!(calc_next(start, 1, 2), 18749137);
    }
}
