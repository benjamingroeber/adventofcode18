use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::Read;

fn main() {
    match run() {
        Ok(checksum) => println!("{}", checksum),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run() -> Result<usize, Box<Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let ids: Vec<_> = input.lines().collect();

    Ok(list_checksum(&ids))
}

fn list_checksum(ids: &[&str]) -> usize {
    let (twos, threes) = ids.iter().fold((0, 0), |(mut twos, mut threes), id| {
        let counters = id.chars().fold(HashMap::new(), |mut counters, char| {
            {
                let count = counters.entry(char).or_insert(0);
                *count += 1;
            }
            counters
        });
        if counters.iter().any(|(_, count)| *count == 2) {
            twos += 1
        }
        if counters.iter().any(|(_, count)| *count == 3) {
            threes += 1
        }
        (twos, threes)
    });
    twos * threes
}

#[test]
fn test_example() {
    let input = vec![
        "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
    ];
    let output = list_checksum(&input);
    assert_eq!(output, 12)
}
