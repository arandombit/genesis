struct Cell {
  alive: bool
}

struct Grid {
  height: usize,
  width: usize,
  cells: Vec<Vec<Cell>>
}

fn main() {
  println!("Initiating genesis...");
  let mut grid = Grid { height: 1024, width: 1024, cells: Vec::new() }
}
