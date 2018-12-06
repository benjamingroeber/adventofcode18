use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::io::Read;
use std::str::FromStr;

fn main() {
    if let Err(e) = run() {
        println!("FATAL ERROR: {}", e)
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let destinations: Result<Vec<_>, _> =
        input.lines().map(|line| FromStr::from_str(line)).collect();
    let grid = SubGrid::new(&destinations?)?;

    // part1
    let a = grid.finite_area();
    let mut v = a.iter().collect::<Vec<_>>();
    v.sort_by_key(|x| x.1);
    println!("{:#?}", v);
    if let Some((largest, count)) = grid.finite_area().iter().max_by_key(|area| area.1) {
        println!(
            "The Destination {:?} has the largest area of {}.",
            largest, count
        );
    } else {
        return Err(From::from("Could not determine largest area"));
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
        let parts: Result<Vec<usize>, _> = s.split(",").map(|p| p.trim().parse()).collect();
        match parts?.as_slice() {
            [x, y] => Ok(Destination { x: *x, y: *y }),
            _ => Err(From::from("Could not parse Destination")),
        }
    }
}

#[derive(Clone, Debug)]
struct SubGrid {
    size_x: usize,
    size_y: usize,
    distances: Vec<(HashMap<Destination, Distance>, Option<Destination>)>,
    bordering: HashSet<Destination>,
}

impl SubGrid {
    fn new(destinations: &[Destination]) -> Result<Self, Box<Error>> {
        if destinations.is_empty() {
            return Err(From::from("Destinations may not be empty!"));
        }
        let size_x = destinations.iter().map(|d| d.x).max().unwrap();
        let size_y = destinations.iter().map(|d| d.y).max().unwrap();

        let mut distances = Vec::new();
        let mut bordering = HashSet::new();
        for y in 0..=size_y {
            for x in 0..=size_x {
                let destination_distances: HashMap<_, _> = destinations
                    .iter()
                    .map(|d| (d.to_owned(), d.manhattan_distance(x, y)))
                    .collect();
                let min_distance = destination_distances.iter().min_by_key(|d| d.1).unwrap();
                let min_count = destination_distances
                    .iter()
                    .filter(|d| *d.1 == *min_distance.1)
                    .count();
                match min_count {
                    1 => {
                        distances.push((
                            destination_distances.to_owned(),
                            Some(min_distance.0.to_owned()),
                        ));
                        if x == 0 || x == size_x || y == 0 || y == size_y {
                            bordering.insert(min_distance.0.to_owned());
                        }
                    }
                    _ => distances.push((destination_distances.to_owned(), None)),
                }
            }
        }
        Ok(Self {
            size_x,
            size_y,
            distances,
            bordering,
        })
    }

    fn finite_area(&self) -> HashMap<Destination, usize> {
        self.distances
            .iter()
            .filter_map(|d| d.to_owned().1.or(None))
            .filter(|d| !self.bordering.contains(d))
            .fold(HashMap::new(), |mut acc, item| {
                acc.entry(item).and_modify(|entry| *entry += 1).or_insert(1);
                acc
            })
    }

    fn rows(
        &self,
    ) -> impl Iterator<Item = &[(HashMap<Destination, Distance>, Option<Destination>)]> {
        self.distances.chunks(self.size_x)
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
        Destination { x: 1, y: 1 },
        Destination { x: 1, y: 6 },
        Destination { x: 8, y: 3 },
        Destination { x: 3, y: 4 },
        Destination { x: 5, y: 5 },
        Destination { x: 8, y: 9 },
    ];

    let subgrid = SubGrid::new(&input).unwrap();
    let areas = subgrid.finite_area();
    println!("{:#?}", subgrid);
    println!("{:#?}", areas);
}
