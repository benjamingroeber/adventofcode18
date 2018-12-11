use std::error::Error;

// THIS IS THE TEST INPUT
const GRID_SERIAL: GridSerial = 2187;

fn main() {
    if let Err(e) = run() {
        eprintln!("FATAL ERROR: {}", e)
    }
}
fn run() -> Result<(), Box<Error>> {
    let grid = Grid::new(GRID_SERIAL, 300, 300);
    let subgrid_size = 3;

    let mut subgrid_power: Vec<(usize, usize, PowerLevel)> = Vec::new();
    for y in 0..grid.size_y - subgrid_size {
        for x in 0..grid.size_x - subgrid_size {
            let subgrid = grid.sub_grid(x, y, subgrid_size, subgrid_size)?;
            let powerlevel = subgrid.iter().map(|n|n.power_level()).sum();
            subgrid_power.push((subgrid[0].x, subgrid[0].y, powerlevel));
        }
    }
    let max_power = subgrid_power
        .iter()
        .max_by_key(|n| n.2)
        .ok_or("Could not determine maximum power level of sub-grids")?;

    println!(
        "The Subgrid with the most stored power of {} starts at {},{}",
        max_power.2, max_power.0, max_power.1
    );
    Ok(())
}

type PowerLevel = i32;
type GridSerial = i32;

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
    ) -> Result<Vec<&Node>, Box<Error>> {
        if self.size_x < from_x + size_x || self.size_y < from_y + size_y {
            return Err(From::from("Subgrid must be fully inside grid"));
        }

        let mut subgrid = Vec::with_capacity(size_y * size_x);
        for offset_y in 0..size_y {
            for offset_x in 0..size_x {
                let y = from_x + offset_y;
                let x = from_y + offset_x;

                subgrid.push(&self.nodes[y * self.size_x + x])
            }
        }
        Ok(subgrid)
    }
}

#[derive(Copy, Debug, Clone)]
struct Node {
    grid_serial: i32,
    x: usize,
    y: usize,
}

impl Node {
    fn new(x: usize, y: usize, grid_serial: GridSerial) -> Self {
        Node { grid_serial, x, y }
    }
    fn rack_id(&self) -> usize {
        self.x + 10
    }
    fn power_level(&self) -> PowerLevel {
        let rack_id = self.rack_id() as PowerLevel;
        let y = self.y as PowerLevel;
        // rack ID * y coordinate, then plus GRID_SERIAL, then * rack ID
        let power = rack_id * (self.grid_serial + rack_id * y);
        // hundreds digit or zero
        let power_level = if power > 100 { (power / 100) % 10 } else { 0 };

        // final result minus 5
        power_level - 5
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
        assert_eq!(4, node.power_level());
    }

    #[test]
    fn test_power_level_examples() {
        let tests: &[(usize, usize, GridSerial, PowerLevel)] =
            &[(122, 79, 57, -5), (217, 196, 39, 0), (101, 153, 71, 4)];

        for (x, y, grid_serial, expected) in tests.iter() {
            let node = Node::new(*x, *y, *grid_serial);
            assert_eq!(*expected, node.power_level())
        }
    }
}
