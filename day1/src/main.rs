extern crate core;

use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::io::Read;

fn main() {
    let changes = parse_changes_from_stdin();
    match &changes {
        Ok(changes) => {
            println!("Frequency: {}", frequency(changes));
            println!("Stable Frequency: {}", stable_frequency(changes))
        }
        Err(e) => eprintln!("Could not read changes from stdin: {}", e),
    }
}

fn parse_changes_from_stdin() -> Result<Vec<i64>, Box<Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    input
        .lines()
        .map(|line| line.parse().map_err(|e| Box::from(e)))
        .collect()
}

fn frequency(input: &[i64]) -> i64 {
    input.iter().sum()
}

fn stable_frequency(input: &[i64]) -> i64 {
    let mut seen = HashSet::new();
    let mut current = 0;
    seen.insert(current);

    for change in input.iter().cycle() {
        current += change;

        if seen.contains(&current) {
            return current;
        } else {
            seen.insert(current);
        }
    }
    unreachable!("Loop is infinite or returns.")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_linear_pos() {
        let input = vec![1, 1, 1];
        let result = frequency(&input);
        assert_eq!(result, 3)
    }

    #[test]
    fn test_sum_0() {
        let input = vec![1, 1, -2];
        let result = frequency(&input);
        assert_eq!(result, 0)
    }

    #[test]
    fn test_linear_neg() {
        let input = vec![-1, -2, -3];
        let result = frequency(&input);
        assert_eq!(result, -6)
    }
    // part two
    #[test]
    fn test_twice_0() {
        let input = vec![1, -1];
        let result = stable_frequency(&input);
        assert_eq!(result, 0)
    }
    #[test]
    fn test_twice_10() {
        let input = vec![3, 3, 4, -2, -4];
        let result = stable_frequency(&input);
        assert_eq!(result, 10)
    }
    #[test]
    fn test_twice_5() {
        let input = vec![-6, 3, 8, 5, -6];
        let result = stable_frequency(&input);
        assert_eq!(result, 5)
    }
    #[test]
    fn test_twice_14() {
        let input = vec![7, 7, -2, -7, -4];
        let result = stable_frequency(&input);
        assert_eq!(result, 14)
    }
}
