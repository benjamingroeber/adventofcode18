use rayon::prelude::*;
use std::error::Error;
use indicatif::ProgressBar;

// THIS IS THE TEST INPUT
const GRID_SERIAL: GridSerial = 2187;
const GRID_SIZE: usize = 300;

fn main() {
    if let Err(e) = run() {
        eprintln!("FATAL ERROR: {}", e)
    }
}

fn run() -> Result<(), Box<Error>> {
    let grid = Grid::new(GRID_SERIAL, GRID_SIZE, GRID_SIZE);

    let most_powerful_subgrids = most_powerful_subgrid_for_square_sizes(&grid, GRID_SIZE);

    // part 1 requests 3x3
    let part1_grid = most_powerful_subgrids[2];
    println!(
        "The Subgrid of size {}x{} with the most stored power of {} starts at {},{}",
        part1_grid.size_x,
        part1_grid.size_y,
        part1_grid.power_level_sum,
        part1_grid.origin_x,
        part1_grid.origin_y
    );

    // part 2
    let most_powerful_grid = most_powerful_subgrids
        .iter()
        .max_by_key(|grid| grid.power_level_sum)
        .ok_or("Could not determine maximum subgrid power level")?;
    println!(
        "The Overall most powerfull subgrid is of size {}x{} with the most stored power of {} staring at {},{}",
        most_powerful_grid.size_x, most_powerful_grid.size_y, most_powerful_grid.power_level_sum, most_powerful_grid.origin_x, most_powerful_grid.origin_y
    );

    Ok(())
}

fn most_powerful_subgrid_for_square_sizes(grid: &Grid, size: usize) -> Vec<SubGrid> {
    let bar = ProgressBar::new(size as u64);
    let max_power_for_subgrid_size: Vec<_> = (1..size)
        .into_par_iter()
        .map(|subgrid_size| {
            let grids = grid
                .all_subgrids_with_size(subgrid_size, subgrid_size)
                .unwrap();

            bar.inc(1);
            *grids.iter().max_by_key(|g| g.power_level_sum).unwrap()
        })
        .collect();
    bar.finish();
    max_power_for_subgrid_size
}

type PowerLevel = i32;
type GridSerial = i32;

#[derive(Copy, Clone,Debug)]
struct SubGrid {
    origin_x: usize,
    origin_y: usize,
    size_x: usize,
    size_y: usize,
    power_level_sum: PowerLevel,
}

#[derive(Debug, Clone)]
struct Node {
    grid_serial: i32,
    x: usize,
    y: usize,
    power_level: PowerLevel,
}

impl Node {
    fn new(x: usize, y: usize, grid_serial: GridSerial) -> Self {
        let power_level = power_level(x, y, grid_serial);
        Node {
            grid_serial,
            x,
            y,
            power_level,
        }
    }
}

fn power_level(x: usize, y: usize, grid_serial: GridSerial) -> PowerLevel {
    // x plus 10
    let rack_id = (x + 10) as PowerLevel;
    let y = y as PowerLevel;
    // rack ID * y coordinate, then plus GRID_SERIAL, then * rack ID
    let power = rack_id * (grid_serial + rack_id * y);
    // hundreds digit or zero
    let power_level = if power > 100 { (power / 100) % 10 } else { 0 };

    // final result minus 5
    power_level - 5
}

#[derive(Debug, Clone)]
struct Grid {
    nodes: Vec<Node>,
    size_x: usize,
    size_y: usize,
    grid_serial: GridSerial,
}

impl Grid {
    fn new(grid_serial: GridSerial, size_x: usize, size_y: usize) -> Self {
        let mut nodes = Vec::with_capacity(size_x * size_y);
        for y in 0..size_y {
            for x in 0..size_x {
                nodes.push(Node::new(x + 1, y + 1, grid_serial))
            }
        }
        Grid {
            nodes,
            size_x,
            size_y,
            grid_serial,
        }
    }
    fn sub_grid(
        &self,
        from_x: usize,
        from_y: usize,
        size_x: usize,
        size_y: usize,
    ) -> Result<SubGrid, Box<Error>> {
        if self.size_x <= from_x + size_x || self.size_y <= from_y + size_y {
            return Err(From::from("Subgrid must be fully inside grid"));
        }

        let mut power_level_sum = 0;
        //        let mut subgrid = Vec::with_capacity(size_y * size_x);
        let node_origin = &self.nodes[from_y * self.size_x + from_x];
        for offset_y in 0..size_y {
            for offset_x in 0..size_x {
                let y = from_y + offset_y;
                let x = from_x + offset_x;

                power_level_sum += self.nodes[y * self.size_x + x].power_level
            }
        }
        Ok(SubGrid {
            origin_x: node_origin.x,
            origin_y: node_origin.y,
            size_x,
            size_y,
            power_level_sum,
        })
    }

    fn all_subgrids_with_size(
        &self,
        size_x: usize,
        size_y: usize,
    ) -> Result<Vec<SubGrid>, Box<Error>> {
        let max_y = self.size_y - size_y;
        let max_x = self.size_y - size_y;

        let mut subgrids: Vec<SubGrid> = Vec::new();
        for y in 0..max_y {
            for x in 0..max_x {
                subgrids.push(self.sub_grid(x, y, size_x, size_y)?);
            }
        }
        Ok(subgrids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This is the puzzle input
    const TEST_SERIAL: GridSerial = 8;

    #[test]
    fn test_power_level() {
        let node = Node::new(3, 5, TEST_SERIAL);
        assert_eq!(4, node.power_level);
    }

    #[test]
    fn test_power_level_examples() {
        let tests: &[(usize, usize, GridSerial, PowerLevel)] =
            &[(122, 79, 57, -5), (217, 196, 39, 0), (101, 153, 71, 4)];

        for (x, y, grid_serial, expected) in tests.iter() {
            let node = Node::new(*x, *y, *grid_serial);
            assert_eq!(*expected, node.power_level)
        }
    }
}
