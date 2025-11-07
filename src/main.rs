#[derive(Clone)]
struct Cell {
  alive: bool
}

struct Grid {
  height: usize,
  width: usize,
  cells: Vec<Vec<Cell>>
}

impl Grid {
  fn new(height: usize, width: usize) -> Self {
    let cells = vec![vec![Cell { alive: false }; height]; width];
    Grid { height, width, cells }
  }
}

fn main() {
  println!("Initiating genesis...");
  let mut grid = Grid { height: 1024, width: 1024, cells: Vec::new() }
}
