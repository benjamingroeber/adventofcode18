use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::Read;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e)
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let ids: Vec<_> = input.lines().collect();

    let checksum = list_checksum(&ids);
    println!("Checksum: '{}'", checksum);

    let chars_of_close_ids = common_box_id_letters(&ids)?;
    println!("First long common id chars: '{}'", chars_of_close_ids);
    Ok(())
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

fn common_box_id_letters(ids: &[&str]) -> Result<String, Box<Error>> {
    for i in 0..ids.len() {
        for j in i..ids.len() {
            let id1 = ids[i];
            let id2 = ids[j];
            if id1.len() != id2.len() {
                continue;
            }
            let commons: String = id1
                .chars()
                .zip(id2.chars())
                .filter(|(ch1, ch2)| ch1 == ch2)
                .map(|(ch1, _)| ch1)
                .collect();
            if commons.len() == id1.len() - 1 {
                return Ok(commons);
            }
        }
    }
    Err(From::from("No box ids differing by just 1 letter"))
}

#[test]
fn test_example() {
    let input = vec![
        "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
    ];
    let output = list_checksum(&input);
    assert_eq!(output, 12)
}

#[test]
fn test_common_example(){
    let input = vec!["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"];
    let result = common_box_id_letters(&input);

    let output = result.unwrap();
    assert_eq!(output, "fgij")
}