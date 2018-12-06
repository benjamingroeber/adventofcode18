extern crate core;

use core::fmt;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
use std::io;
use std::io::Read;

fn main() {
    if let Err(e) = run() {
        println!("FATAL ERROR: {}", e)
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let destinations: Result<Vec<_>,_> = input.lines().map(|line|FromStr::from_str(line)).collect();
    let grid = SubGrid::new(&destinations?)?;

    // part1
    let a = grid.finite_areas();
    let mut v = a.iter().collect::<Vec<_>>();
    v.sort_by_key(|x|x.1);
    println!("{:#?}",v);
    if let Some((largest, count)) = grid.finite_areas().iter().max_by_key(|area| area.1) {
        println!("The Destination {:?} has the largest area of {}.", largest, count)
    } else {
        return Err(From::from("Could not determine largest area"))
    }

    Ok(())
}

type Distance = usize;
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
struct Destination {
    x: usize,
    y: usize,
}

impl FromStr for Destination {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let parts : Result<Vec<usize>,_> = s.split(",").map(|p|p.trim().parse()).collect();
        match parts?.as_slice() {
            [x,y] => Ok(Destination{x:*x,y:*y}),
            _ => Err(From::from("Could not parse Destination"))
        }
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
enum Closest {
    Destination(Destination, Distance),
    Multi,
}

#[derive(Clone, Debug)]
struct SubGrid {
    size_x: usize,
    size_y: usize,
    closest: Vec<Closest>,
}

impl SubGrid {
    fn new(destinations: &[Destination]) -> Result<Self, Box<Error>> {
        if destinations.is_empty() {
            return Err(From::from("Destinations may not be empty!"));
        }
        let size_x = destinations.iter().map(|d| d.x).max().unwrap();
        let size_y = destinations.iter().map(|d| d.y).max().unwrap();

        let mut closest = Vec::new();
        for y in 0..=size_y {
            for x in 0..=size_x {
                let distances = destinations
                    .iter()
                    .map(|d| (d, d.manhattan_distance(x, y)))
                    .collect::<Vec<_>>();
                let min_distance = distances.iter().min_by_key(|d| d.1).unwrap();
                let min_distance_count = distances.iter().filter(|i| i.1 == min_distance.1).count();
                match min_distance_count {
                    0 => return Err(From::from("Point can not have no distances")),
                    1 => closest.push(Closest::Destination(
                        min_distance.0.to_owned(),
                        min_distance.1,
                    )),
                    _ => closest.push(Closest::Multi),
                }
            }
        }

        Ok(Self {
            size_x,
            size_y,
            closest,
        })
    }

    fn rows(&self) -> impl Iterator<Item = &[Closest]> {
        self.closest.chunks(self.size_y)
    }

    fn finite_areas(&self) -> HashMap<Destination, usize> {
        let areas = self
            .closest
            .iter()
            .filter_map(|c| {
                if let Closest::Destination(dest, _) = c {
                    Some(dest.to_owned())
                } else {
                    None
                }
            }).fold(HashMap::new(), |mut h_map, dest| {
                h_map
                    .entry(dest)
                    .and_modify(|entry| *entry += 1)
                    .or_insert(1);
                h_map
            });
        areas
    }
}

impl fmt::Display for SubGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.rows() {
            write!(f, "\n");
            for closest in row {
                match closest {
                    Closest::Destination(destination, ..) => {
                        write!(f, "({:03},{:03}) ", destination.x, destination.y)
                    }
                    Closest::Multi => write!(f, "( MULTI ) "),
                }?;
            }
        }
        Ok(())
    }
}

fn manhattan_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    let x_dist = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    let y_dist = if y1 > y2 { y1 - y2 } else { y2 - y1 };
    x_dist + y_dist
}

impl Destination {
    fn manhattan_distance(&self, x: usize, y: usize) -> usize {
        manhattan_distance(self.x, self.y, x, y)
    }
}

#[test]
fn test_example() {
    let input = vec![
        Destination {
            x: 1,
            y: 1,
        },
        Destination {
            x: 1,
            y: 6,
        },
        Destination {
            x: 8,
            y: 3,
        },
        Destination {
            x: 3,
            y: 4,
        },
        Destination {
            x: 5,
            y: 5,
        },
        Destination {
            x: 8,
            y: 9,
        },
    ];
    let subgrid = SubGrid::new(&input).unwrap();

    let areas = subgrid.finite_areas();
    let max = areas.iter().max_by_key(|a|a.1).unwrap();

    assert_eq!(17,*max.1)

}
