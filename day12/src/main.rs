use std::error::Error;
use std::fmt;
use std::io;
use std::io::Read;
use std::str::FromStr;

fn main() {
    if let Err(e) = run() {
        eprintln!("FATAL ERROR: {}", e)
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // parse initial config
    let mut lines = input.lines();
    let initial_config = lines
        .next()
        .map(|line| {
            line.chars()
                .filter(|c| is_valid_pot(*c))
                .map(char_as_pot)
                .collect::<Result<Vec<_>, _>>()
        })
        .ok_or("Could not get initial config")??;

    let mut pots = Pots::new(initial_config);

    // parse rules
    let rules: Vec<Rule> = lines
        .filter(|l| !l.is_empty())
        .map(FromStr::from_str)
        .collect::<Result<_, _>>()?;

    // First 20 iterations
    for _i in 0..20 {
        println!("{}", pots);
        pots.next_generation(&rules);
    }
    println!(
        "Sum of pot numbers after 20 iterations: {}",
        pots.sum_full_pot_numbers()
    );

    Ok(())
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Pot {
    Empty,
    Full,
}

#[derive(Debug, Clone)]
struct Pots {
    leftmost_number: i32,
    generation: usize,
    pots: Vec<Pot>,
}

const ALIGNMENT: usize = 4;

impl Pots {
    fn new(pots: Vec<Pot>) -> Self {
        let mut pots = Pots {
            leftmost_number: 0,
            generation: 0,
            pots,
        };

        pots.align();
        pots
    }

    // sum the numbers written on the full pots
    fn sum_full_pot_numbers(&self) -> i32 {
        self.pots
            .iter()
            .enumerate()
            .filter(|(_, p)| **p == Pot::Full)
            .map(|(i, _)| self.leftmost_number + i as i32)
            .sum()
    }

    // the pattern ....# and #.... must be able to be matched,
    // so we need at least 4 empty Pots at each side
    // also we need at least 5 pots to be able to calculate something useful
    fn align(&mut self) {
        while self.pots.len() < 5 {
            self.pots.push(Pot::Empty)
        }
        while self.pots[0..ALIGNMENT].iter().any(|p| *p == Pot::Full) {
            self.leftmost_number -= 1;
            self.pots.insert(0, Pot::Empty)
        }
        while self.pots[self.pots.len() - ALIGNMENT..self.pots.len()]
            .iter()
            .any(|p| *p == Pot::Full)
        {
            self.pots.push(Pot::Empty)
        }
    }

    fn next_generation(&mut self, rules: &[Rule]) {
        let old = self.pots.clone();
        for i in 2..old.len() - 2 {
            let pattern = &old[i - 2..=i + 2];
            for rule in rules {
                if pattern == rule.pattern {
                    self.pots[i] = rule.next
                }
            }
        }
        self.align();
        self.generation += 1
    }
}

impl fmt::Display for Pots {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}: ", self.generation)?;
        for pot in &self.pots {
            match pot {
                Pot::Empty => write!(f, ".")?,
                Pot::Full => write!(f, "#")?,
            }
        }
        Ok(())
    }
}

// A pattern of 5 Pots describes the state of the middle pot in the next generation
#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
    pattern: [Pot; 5],
    next: Pot,
}

impl FromStr for Rule {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut pattern = [Pot::Empty; 5];
        let mut chars = s.chars().filter(|c| is_valid_pot(*c));
        for i in 0..5 {
            if let Some(c) = chars.next() {
                if c == '#' {
                    pattern[i] = Pot::Full
                }
            } else {
                return Err(From::from("Not enough input for pattern"));
            }
        }

        let next = chars
            .next()
            .map(char_as_pot)
            .ok_or("Not enough input for next generation")??;

        Ok(Rule { pattern, next })
    }
}

fn is_valid_pot(c: char) -> bool {
    c == '#' || c == '.'
}

fn char_as_pot(c: char) -> Result<Pot, Box<Error>> {
    match c {
        '#' => Ok(Pot::Full),
        '.' => Ok(Pot::Empty),
        _ => Err(From::from("Pot pattern not recognized")),
    }
}

#[test]
fn test_parse_pot() {
    assert_eq!(Pot::Empty, char_as_pot('.').unwrap());
    assert_eq!(Pot::Full, char_as_pot('#').unwrap());

    assert!(char_as_pot(' ').is_err());
    assert!(char_as_pot('=').is_err());
    assert!(char_as_pot('>').is_err());
}

#[test]
fn test_parse_rule() {
    let rule: Rule = FromStr::from_str("..### => #").unwrap();
    assert_eq!(
        Rule {
            pattern: [Pot::Empty, Pot::Empty, Pot::Full, Pot::Full, Pot::Full],
            next: Pot::Full
        },
        rule
    )
}

#[test]
fn test_align_extend() {
    let pots = vec![Pot::Full];
    let pots = Pots::new(pots);

    assert_eq!(pots.leftmost_number, -4);
    assert_eq!(pots.pots.len(), 9)
}
