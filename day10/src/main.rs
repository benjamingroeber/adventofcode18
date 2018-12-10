use regex::Regex;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::Read;
use std::str::FromStr;

#[macro_use]
extern crate lazy_static;

fn main() {
    if let Err(e) = run() {
        eprintln!("FATAL ERROR: {}", e)
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let point_list: Result<Vec<Point>, _> =
        input.lines().map(|line| FromStr::from_str(line)).collect();
    let mut points = Points::new(point_list?);

    let mut i = 0;
    let mut last_size = i64::max_value();
    loop {
        points.step();
        let dimensions = points.dimensions()?;
        let current_size =
            (dimensions.max_x - dimensions.min_x) + (dimensions.max_y - dimensions.min_y);
        if current_size > last_size {
            break;
        }
        i += 1;
        last_size = current_size;
    }
    points.step_back();
    println!("{}s:{}\n\n", i, points);
    Ok(())
}

struct Points(Vec<Point>);

#[derive(Debug, Clone)]
struct Dimensions {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Points {
    fn new(points: Vec<Point>) -> Points {
        Points(points)
    }

    fn dimensions(&self) -> Result<Dimensions, Box<Error>> {
        let points = &self.0;
        let max_x = points
            .iter()
            .max_by_key(|p| p.x)
            .map(|p| p.x)
            .ok_or("Could not get max_x")?;
        let min_x = points
            .iter()
            .min_by_key(|p| p.x)
            .map(|p| p.x)
            .ok_or("Could not get max_x")?;
        let max_y = points
            .iter()
            .max_by_key(|p| p.y)
            .map(|p| p.y)
            .ok_or("Could not get max_x")?;
        let min_y = points
            .iter()
            .min_by_key(|p| p.y)
            .map(|p| p.y)
            .ok_or("Could not get max_x")?;

        Ok(Dimensions {
            min_x,
            max_x,
            min_y,
            max_y,
        })
    }

    fn step(&mut self) {
        for point in &mut self.0 {
            point.x += i64::from(point.vel_x);
            point.y += i64::from(point.vel_y);
        }
    }

    fn step_back(&mut self) {
        for point in &mut self.0 {
            point.x -= i64::from(point.vel_x);
            point.y -= i64::from(point.vel_y);
        }
    }
}

impl fmt::Display for Points {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        // TODO convert do fmt::error
        let dimensions = self.dimensions().unwrap();

        // len(), is max_idx + 1
        let len_x = 1 + dimensions.max_x - dimensions.min_x;
        let len_y = 1 + dimensions.max_y - dimensions.min_y;

        // as min_y is at most == max_y, len_y can't ever be less than 1
        assert!(len_x > 0);
        // as min_y is at most == max_y, len_y can't ever be less than 1
        assert!(len_y > 0);

        let mut grid = vec![false; (len_x * len_y) as usize];
        for point in &self.0 {
            // shift x and y such that 0,0 indexes the origin
            let real_x = point.x - dimensions.min_x.abs();
            let real_y = point.y - dimensions.min_y.abs();

            let idx = real_y * len_x + real_x;
            grid[idx as usize] = true;
        }

        for (i, cell) in grid.iter().enumerate() {
            // newline or space to separate rows/values
            let sep = if i % (len_x as usize) == 0 { '\n' } else { ' ' };
            write!(f, "{}", sep)?;

            // █  for Value, • for no value
            let mark = if *cell { '█' } else { '•' };

            write!(f, "{}", mark)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
    vel_x: i32,
    vel_y: i32,
}

lazy_static! {
    static ref PAIR_RX: Regex = {
        Regex::new(r"position=<\s*(?P<x>[-0-9]+)\s*,\s*(?P<y>[-0-9]+)\s*> velocity=<\s*(?P<vel_x>[-0-9]+)\s*,\s*(?P<vel_y>[-0-9]+)\s*>").unwrap()
    };
}

impl FromStr for Point {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let caps = PAIR_RX
            .captures(s)
            .ok_or("Could not extract position and velocity")?;

        let x = caps
            .name("x")
            .ok_or("Could not extract x")
            .map(|cap| cap.as_str().parse::<i64>())??;
        let y = caps
            .name("y")
            .ok_or("Could not extract y")
            .map(|cap| cap.as_str().parse::<i64>())??;
        let vel_x = caps
            .name("vel_x")
            .ok_or("Could not extract vel_x")
            .map(|cap| cap.as_str().parse::<i32>())??;
        let vel_y = caps
            .name("vel_y")
            .ok_or("Could not extract vel_y")
            .map(|cap| cap.as_str().parse::<i32>())??;

        Ok(Point { x, y, vel_x, vel_y })
    }
}

#[test]
fn test_print() {
    let mut points = Vec::new();
    for x in -20..=20 {
        for y in -10..=10 {
            if y % 5 != 0 && (x % 3 == y || y > 5 || y < -5) {
                points.push(Point {
                    x,
                    y,
                    vel_x: 0,
                    vel_y: 0,
                })
            }
        }
    }

    let points = Points::new(points);
    eprintln!("{}", points);

    assert!(true)
}

#[test]
fn test_parse_point() {
    let input = "position=< 9,  1> velocity=< 0,  2>\nposition=< 7,  0> velocity=<-1,  0>\nposition=< 3, -2> velocity=<-1,  1>";

    let points: Result<Vec<Point>, _> = input.lines().map(|line| FromStr::from_str(line)).collect();

    let points = points.unwrap();

    assert_eq!(
        points.as_slice(),
        &[
            Point {
                x: 9,
                y: 1,
                vel_x: 0,
                vel_y: 2
            },
            Point {
                x: 7,
                y: 0,
                vel_x: -1,
                vel_y: 0
            },
            Point {
                x: 3,
                y: -2,
                vel_x: -1,
                vel_y: 1,
            },
        ]
    )
}
