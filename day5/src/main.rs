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

    let polymer = Polymer::new(&input)?;

    // Part 1
    let reduction = polymer.reduce(None);
    println!("Reduction (Len: {}): {}", reduction.len(), reduction);

    // Part 2
    let alphabet = b'a'..=b'z';
    if let Some((unit, reduction)) = alphabet
        .map(|unit| (unit, polymer.reduce(Some(unit as char))))
        .min_by_key(|(_, reduced)| reduced.len())
    {
        println!(
            "Maximum reduction to length {} possible by removing unit {}",
            reduction.len(),
            unit as char
        );
    } else {
        return Err(From::from("Could not determine shortest reduction"));
    }

    Ok(())
}

type Unit = char;
#[derive(Debug, Clone, PartialEq)]
struct Polymer(String);
impl Polymer {
    fn new<T: AsRef<str> + ?Sized>(s: &T) -> Result<Polymer, Box<Error>> {
        let non_ascii_alphabetic = s
            .as_ref()
            .chars()
            .filter(|c| !c.is_alphabetic() || !c.is_ascii())
            .count();
        if non_ascii_alphabetic > 0 {
            Err(From::from(
                "Input contains non alphabetic or non ascii characters!",
            ))
        } else {
            Ok(Polymer(s.as_ref().to_owned()))
        }
    }

    // by new() we are guaranteed to contain only ascii alphabetic characters
    fn reduce(&self, ignore_unit: Option<Unit>) -> ReducedPolymer {
        let ignore_unit = ignore_unit.map(|unit| unit.to_ascii_lowercase());

        let not_ignored_units = self
            .0
            .chars()
            // if given, filter units matching the ignore_units from iterator
            .filter(|c| {
                ignore_unit
                    .map(|ignore| ignore.to_ascii_lowercase() != c.to_ascii_lowercase())
                    .unwrap_or(true)
            });

        let mut polymer = String::new();
        for current in not_ignored_units {
            if let Some(last) = polymer.chars().last() {
                // remove last pushed unit, and ignore current unit, if they match
                if current != last && current.to_ascii_lowercase() == last.to_ascii_lowercase() {
                    polymer.pop();
                    continue;
                }
            }
            polymer.push(current);
        }
        ReducedPolymer(polymer)
    }
}

impl FromStr for Polymer {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Polymer::new(s)
    }
}
#[derive(Debug, Clone, PartialEq)]
struct ReducedPolymer(String);

impl ReducedPolymer {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for ReducedPolymer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn test_failed_polymer() {
    let poly_start = Polymer::new(">aAaasA");
    let poly_center = Polymer::new("aAa!asA");
    let poly_end = Polymer::new("aAaasAü");

    assert!(poly_start.is_err());
    assert!(poly_center.is_err());
    assert!(poly_end.is_err());
}

#[test]
fn test_new_polymer() {
    let polymer = Polymer::new("aAa!asA");

    assert!(polymer.is_err())
}

#[test]
fn test_full_reduction() {
    let polymer = Polymer::new("aA").unwrap();

    let reduced = polymer.reduce(None);

    assert_eq!(Polymer("aA".to_owned()), polymer);
    assert_eq!(ReducedPolymer("".to_owned()), reduced);
}

#[test]
fn test_recursive_reduction() {
    let polymer = Polymer::new("abBA").unwrap();

    let reduced = polymer.reduce(None);

    assert_eq!(Polymer("abBA".to_owned()), polymer);
    assert_eq!(ReducedPolymer("".to_owned()), reduced);
}

#[test]
fn test_no_reduction() {
    let polymer = Polymer::new("abAB").unwrap();

    let reduced = polymer.reduce(None);

    assert_eq!(Polymer("abAB".to_owned()), polymer);
    assert_eq!(ReducedPolymer("abAB".to_owned()), reduced);
}

#[test]
fn test_multi_no_reduction() {
    let polymer = Polymer::new("aabAAB").unwrap();

    let reduced = polymer.reduce(None);

    assert_eq!(Polymer("aabAAB".to_owned()), polymer);
    assert_eq!(ReducedPolymer("aabAAB".to_owned()), reduced)
}

#[test]
fn test_example() {
    let polymer = Polymer::new("dabAcCaCBAcCcaDA").unwrap();

    let reduced = polymer.reduce(None);

    assert_eq!(Polymer("dabAcCaCBAcCcaDA".to_owned()), polymer);
    assert_eq!(ReducedPolymer("dabCBAcaDA".to_owned()), reduced)
}

#[test]
fn test_example_reduce_ignoring_unit() {
    let a_poly = Polymer::new("dabAcCaCBAcCcaDA").unwrap();
    let b_poly = Polymer::new("dabAcCaCBAcCcaDA").unwrap();
    let c_poly = Polymer::new("dabAcCaCBAcCcaDA").unwrap();
    let d_poly = Polymer::new("dabAcCaCBAcCcaDA").unwrap();

    let a_reduced = a_poly.reduce(Some('a'));
    let b_reduced = b_poly.reduce(Some('b'));
    let c_reduced = c_poly.reduce(Some('c'));
    let d_reduced = d_poly.reduce(Some('d'));

    assert_eq!(a_reduced.len(), 6);
    assert_eq!(b_reduced.len(), 8);
    assert_eq!(c_reduced.len(), 4);
    assert_eq!(d_reduced.len(), 6);
}
