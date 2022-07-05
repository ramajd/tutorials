use std::{collections::HashSet, fs::File, io::Read};

fn main() {
    let mut file = File::open("input.txt").expect("Failed to open input.txt");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Failed to read input");

    let packages: Vec<usize> = data.lines().map(|l| l.parse().unwrap()).collect();
    println!("{:?}", packages);

    let (_, s) = calculate(&packages, 3);
    println!("Optimal for [3] = {}", s);

    let (_, s) = calculate(&packages, 4);
    println!("Optimal for [4] = {}", s);
}

fn generate_variants(packages: &Vec<usize>, weight: usize, len: usize) -> Vec<Vec<usize>> {
    let mut result = HashSet::new();

    if len > 0 {
        for pkg in packages.clone() {
            if pkg == weight {
                result.insert(vec![pkg]);
            } else if pkg < weight {
                let variants = generate_variants(packages, weight - pkg, len - 1);
                for variant in variants {
                    let variant_weight: usize = variant.iter().sum();
                    if !variant.contains(&pkg) && variant_weight == weight - pkg {
                        let mut variant = variant.clone();
                        variant.push(pkg);
                        variant.sort();
                        variant.reverse();
                        result.insert(variant);
                    }
                }
            }
        }
    }
    // println!("{:?}", result);
    Vec::from_iter(result.into_iter())
}

fn calculate(packages: &Vec<usize>, parts: usize) -> (usize, usize) {
    let target = packages.iter().sum::<usize>() / parts;

    let mut min_len = usize::MAX;
    let mut min_score = usize::MAX;

    for i in 0..packages.len() {
        let variants = generate_variants(packages, target, i);

        for variant in variants {
            // println!("{:?}", variant);
            let l: usize = variant.len();
            let s: usize = variant.iter().product();
            if l < min_len {
                min_len = l;
                min_score = s;
            } else if l == min_len && s < min_score {
                min_score = s;
            }
        }
        if min_len != usize::MAX {
            break;
        }
    }

    (min_len, min_score)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate() {
        let packages = vec![1, 2, 3, 4];
        let result = generate_variants(&packages, 5, 2);
        assert!(result.contains(&vec![4, 1]));
        assert!(result.contains(&vec![3, 2]));
    }

    #[test]
    fn test_divide() {
        let packages = vec![1, 2, 3];
        println!("--- {:?}", packages);
        assert_eq!(calculate(&packages, 2), (1, 3));

        let packages = vec![1, 2, 3, 4];
        println!("--------------- {:?}", packages);
        assert_eq!(calculate(&packages, 2), (2, 4));

        let packages = vec![2, 3, 4, 5, 6];
        println!("--- {:?}", packages);
        assert_eq!(calculate(&packages, 2), (2, 24));

        let packages = vec![1, 2, 3, 4, 5, 7, 8, 10];
        println!("--- {:?}", packages);
        assert_eq!(calculate(&packages, 2), (3, 160));
    }

    #[test]
    fn test_arrangement1() {
        let packages = vec![1, 9, 8, 2, 7, 3];
        assert_eq!(calculate(&packages, 3), (2, 9));
    }

    #[test]
    fn test_arrangement2() {
        let packages = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        assert_eq!(calculate(&packages, 3), (2, 99));
    }
}
