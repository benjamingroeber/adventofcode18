#[macro_use]
extern crate lazy_static;
extern crate rayon;
extern crate regex;

use rayon::prelude::*;
use regex::Regex;
use std::error::Error;
use std::io;
use std::io::Read;
use std::str::FromStr;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e)
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let patches: Result<Vec<Patch>, _> =
        input.lines().map(|line| FromStr::from_str(line)).collect();
    let patches = patches?;

    // part 1
    let mut grid = Grid::new(1000, 1000);
    for patch in &patches {
        grid.claim(patch)?;
    }

    println!("{} squares are overlapping", grid.count_overlapping());

    // TODO Improve part 2
    let mut found = None;
    for patch in &patches {
        let covered = &grid.full_patch_coverage(patch)?;
        if *covered {
            found = Some(patch.id);
            break;
        }
    }

    if let Some(id) = found {
        println!("First non-overlapping patch was '#{}'", id);
    } else {
        eprintln!("No non overlapping patches found!")
    }

    Ok(())
}

#[test]
#[derive(Debug, Clone)]
struct Grid {
    size_x: usize,
    size_y: usize,
    squares: Vec<usize>,
}

impl Grid {
    fn new(size_x: usize, size_y: usize) -> Self {
        let squares = vec![0; size_x * size_y];
        Grid {
            size_x,
            size_y,
            squares,
        }
    }
    fn get(&self, x: usize, y: usize) -> Result<usize, Box<Error>> {
        let idx = self.size_x * y + x;
        if idx > self.squares.len() {
            return Err(From::from("Index out of bounds"));
        }
        Ok(self.squares[idx])
    }
    fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut usize, Box<Error>> {
        let idx = self.size_x * y + x;
        if idx > self.squares.len() {
            return Err(From::from("Index out of bounds"));
        }
        Ok(&mut self.squares[idx])
    }
    fn _claim_cell<'a>(&mut self, x: usize, y: usize) -> Result<(), Box<Error>> {
        let cell = self.get_mut(x, y)?;
        *cell += 1;
        Ok(())
    }
    // Operations on Patches
    fn claim(&mut self, patch: &Patch) -> Result<(), Box<Error>> {
        for x in 0..patch.size_x {
            for y in 0..patch.size_y {
                self._claim_cell(patch.offset_x + x, patch.offset_y + y)?;
            }
        }
        Ok(())
    }
    fn full_patch_coverage(&self, patch: &Patch) -> Result<bool, Box<Error>> {
        for x in 0..patch.size_x {
            for y in 0..patch.size_y {
                let value = self.get(patch.offset_x + x, patch.offset_y + y)?;
                if value != 1 {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }
    fn count_overlapping(&self) -> usize {
        self.squares.par_iter().filter(|count| **count > 1).count()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Patch {
    id: usize,
    offset_x: usize,
    offset_y: usize,
    size_x: usize,
    size_y: usize,
}

// Format:
// #123 @ 3,2: 5x4
lazy_static! {
    static ref FROM_STR_RX: Regex = Regex::new(
        r##"^#(?P<id>[0-9]+)\s+@\s+(?P<offset_x>[0-9]+),(?P<offset_y>[0-9]+):\s+(?P<size_x>[0-9]+)x(?P<size_y>[0-9]+)$"##
    ).unwrap();
}
impl FromStr for Patch {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let caps = FROM_STR_RX.captures(s).ok_or("No Captures")?;

        let id = FromStr::from_str(caps.name("id").ok_or("No id?")?.as_str())?;
        let offset_x = FromStr::from_str(caps.name("offset_x").ok_or("No offset_x?")?.as_str())?;
        let offset_y = FromStr::from_str(caps.name("offset_y").ok_or("No offset_y?")?.as_str())?;
        let size_x = FromStr::from_str(caps.name("size_x").ok_or("No size_x?")?.as_str())?;
        let size_y = FromStr::from_str(caps.name("size_y").ok_or("No size_y?")?.as_str())?;
        Ok(Patch {
            id,
            offset_x,
            offset_y,
            size_x,
            size_y,
        })
    }
}

fn parse_patch() {
    let input = "#123 @ 3,2: 5x4";

    let patch: Patch = FromStr::from_str(input).unwrap();

    let expected = Patch {
        id: 123,
        offset_x: 3,
        offset_y: 2,
        size_x: 5,
        size_y: 4,
    };
    assert_eq!(patch, expected)
}

#[test]
fn claim_origin() {
    let mut grid = Grid::new(2, 2);
    let patch = Patch {
        id: 1,
        offset_y: 0,
        offset_x: 0,
        size_y: 1,
        size_x: 1,
    };

    grid.claim(&patch).unwrap();

    assert_eq!(grid.squares[0], 1);
    assert_eq!(grid.squares[1], 0);
    assert_eq!(grid.squares[2], 0);
    assert_eq!(grid.squares[3], 0);
}

#[test]
fn claim_last() {
    let mut grid = Grid::new(2, 2);
    let patch = Patch {
        id: 1,
        offset_y: 1,
        offset_x: 1,
        size_y: 1,
        size_x: 1,
    };

    grid.claim(&patch).unwrap();

    println!{"{:#?}", grid};
    assert_eq!(grid.squares[0], 0);
    assert_eq!(grid.squares[1], 0);
    assert_eq!(grid.squares[2], 0);
    assert_eq!(grid.squares[3], 1);
}

#[test]
fn claim_all() {
    let mut grid = Grid::new(2, 2);
    let patch = Patch {
        id: 1,
        offset_y: 0,
        offset_x: 0,
        size_y: 2,
        size_x: 2,
    };

    grid.claim(&patch).unwrap();

    assert_eq!(grid.squares[0], 1);
    assert_eq!(grid.squares[1], 1);
    assert_eq!(grid.squares[2], 1);
    assert_eq!(grid.squares[3], 1);
}
