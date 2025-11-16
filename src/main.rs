use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Cell {
  alive: bool
}

#[derive(Debug, Serialize, Deserialize)]
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
  fn neighbor_positions(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::with_capacity(8);
    for dr in [-1isize, 0, 1] {
      for dc in [-1isize, 0, 1] {
        if dr == 0 && dc == 0 { continue; }
        let nr = row as isize + dr;
        let nc = col as isize + dc;
        if (0..self.height as isize).contains(&nr) && (0..self.width as isize).contains(&nc) {
            neighbors.push((nr as usize, nc as usize));
        }
      }
    }
    neighbors
  }
  fn live_neighbor_count(&self, row: usize, col: usize) -> usize {
    self.neighbor_positions(row, col)
      .iter()
      .filter(|&&(r, c)| self.cells[r][c].alive)
      .count()
  }
  fn step(&self) -> Self {
    let mut next_cells = vec![vec![Cell { alive: false }; self.width]; self.height];
    for row in 0..self.height {
      for col in 0..self.width {
        let neighbor_count = self.live_neighbor_count(row, col);
        next_cells[row][col].alive = next_cell_state(self.cells[row][col].alive, neighbor_count);
      }
    }
    Grid { height: self.height, width: self.width, cells: next_cells }
  }
  fn to_json(&self) -> String {
    serde_json::to_string_pretty(self).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
  }
}

fn next_cell_state(current_alive: bool, neighbor_count: usize) -> bool {
  match (current_alive, neighbor_count) {
    (true, 2) | (true, 3) => true,  // Survival
    (true, _) => false,             // Underpopulation or overpopulation
    (false, 3) => true,             // Reproduction
    (false, _) => false,            // Stays dead
  }
}

fn main() {
  println!("Initiating genesis...");
  let mut grid = Grid::new(5, 5);

  let blinker = [(2, 1), (2, 2), (2, 3)];

  for &(row, col) in &blinker {
    grid.cells[row][col].alive = true;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn blinker_center_has_two_neighbors() {
      let mut grid = Grid::new(5, 5);
      for &(r, c) in &[(2, 1), (2, 2), (2, 3)] {
          grid.cells[r][c].alive = true;
      }
      assert_eq!(2, grid.live_neighbor_count(2, 2));
  }
}
