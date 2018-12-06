use std::error::Error;
use std::fmt;

fn main() {
    if let Err(e) = run() {
        println!("FATAL ERROR: {}", e)
    }
}

fn run() -> Result<(), Box<Error>> {
    Ok(())
}

type Distance = usize;
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
struct Destination {
    name: Option<String>,
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
struct SubGrid {
    size_x: usize,
    size_y: usize,
    closest_destination: Vec<Option<Destination>>,
}

impl SubGrid {
    fn new(destinations: &[Destination]) -> Result<Self, Box<Error>> {
        let leftmost = destinations
            .iter()
            .min_by_key(|d| d.x)
            .map(|d| d.x)
            .ok_or("Could not determine leftmost Coordinate")?;
        let rightmost = destinations
            .iter()
            .max_by_key(|d| d.x)
            .map(|d| d.x)
            .ok_or("Could not determine rightmost Coordinate")? ;
        let topmost = destinations
            .iter()
            .min_by_key(|d| d.y)
            .map(|d| d.y)
            .ok_or("Could not determine topmost Coordinate")?;
        let bottommost = destinations
            .iter()
            .max_by_key(|d| d.y)
            .map(|d| d.y)
            .ok_or("Could not determine bottommost Coordinate")?;

        let mut closest_destination = Vec::with_capacity(rightmost*bottommost);
        for y in 0..=rightmost {
            for x in 0..=bottommost {
                let distances: Vec<_> = destinations
                    .iter()
                    .map(|d| (d.to_owned(), d.manhattan_distance(x, y)))
                    .collect();
                if let Some(min_distance) = distances
                    .iter()
                    .min_by_key(|d| d.1) {
                    let min_distance_count = distances.iter().filter(|d| d.1 == min_distance.1).count();
                    match min_distance_count {
                        0 => return Err(From::from("Every field must have a minimum distance!")),
                        1 => closest_destination.push(Some(min_distance.0.to_owned())),
                        _ => closest_destination.push(None),
                    };
                } else {
                    return Err(From::from("Every field must have a minimum distance"));
                }

            }
        }

        Ok(SubGrid {
            size_x: rightmost,
            size_y: bottommost,
            closest_destination,
        })
    }

    fn closest_destination(&self, x: usize, y: usize) -> Option<&Destination> {
        if let Some(got) = self.closest_destination.get(y * self.size_x + x) {
            got.into()
        } else {
            None
        }
    }

    fn rows(&self) -> Vec<&[Option<Destination>]> {
        let mut rows = Vec::new();
        let mut rest = self.closest_destination.as_slice();
        for row in 0..self.size_y {
            let (first, last) =  rest.split_at(self.size_x*row);
            rows.push(first);
            rest = last;
        }
        rows
    }
}

impl fmt::Display for SubGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        for y in 0..=self.size_y {
            write!(f,"\n")?;
            for row in self.rows() {
                write!(f, "\n")?;
                for point in row {
                    if let Some(destination) = point {
                        if let Some(name) = &destination.name {
//                            if destination.x == x && destination.y == y {
//                                write!(f, "{}", name.to_ascii_uppercase())?;
//                            } else {
                                write!(f, "{}", name)?;
//                            }
                        } else {
                            write!(f, "({},{})", destination.x, destination.y)?;
                        }

                    } else {
                        write!(f, ".")?;
                    }
                }
//            }
        };
        Ok(())
    }
}

impl Destination {
    fn manhattan_distance(&self, x: usize, y: usize) -> usize {
        let x_dist = if x > self.x { x - self.x } else { self.x - x };
        let y_dist = if y > self.y { y - self.y } else { self.y - y };
        x_dist + y_dist
    }
}

#[test]
fn test_example() {
    let input = vec![
        Destination {name: Some("a".into()), x: 1, y: 1 },
        Destination {name: Some("b".into()), x: 1, y: 6 },
        Destination {name: Some("c".into()), x: 8, y: 3 },
        Destination {name: Some("d".into()), x: 3, y: 4 },
        Destination {name: Some("e".into()), x: 5, y: 5 },
        Destination {name: Some("f".into()), x: 8, y: 9 },
    ];

    let subgrid = SubGrid::new(&input).unwrap();
    println!("{:#?}\n{}\n{}", subgrid, subgrid,subgrid.closest_destination.len());
    let dest = subgrid.closest_destination(0,0).unwrap();

    assert_eq!(input[0], dest.to_owned())
}
