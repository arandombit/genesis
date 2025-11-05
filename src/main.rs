struct Cell {
  alive: bool
}

struct Grid {
  height: usize,
  width: usize,
  cells: Vec<Vec<Cell>>
}

fn main() {
  let mut grid = Vec::new();
  println!("Hello, world!");
}
