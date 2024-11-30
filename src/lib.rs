use itertools::iproduct;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<'a, T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: vec![T::default(); rows * cols],
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        assert_eq!(grid.len(), rows * cols);
        Self {
            rows,
            cols,
            grid: grid.to_vec(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        assert!(row < self.rows);
        assert!(col < self.cols);
        &self.grid[col * self.rows + row]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        assert!(row < self.rows);
        assert!(col < self.cols);
        self.grid[col * self.rows + row] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        assert!(row < self.rows);
        assert!(col < self.cols);

        iproduct!(
            row.saturating_sub(1)..=row + 1,
            col.saturating_sub(1)..=col + 1,
        )
        .filter(move |(r, c)| r < &self.rows && c < &self.cols && (r, c) != (&row, &col))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        Self { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn step(&mut self) {
        let grid_size = self.grid.size();
        let mut new_grid = Grid::new(grid_size.0, grid_size.1);
        for (row, col) in iproduct!(0..grid_size.0, 0..grid_size.1) {
            let mut live_cells: u32 = 0;
            for (neighbour_row, neighbour_col) in self.grid.neighbours(row, col) {
                live_cells += {
                    match self.grid.get(neighbour_row, neighbour_col) {
                        Cell::Dead => 0,
                        Cell::Alive => 1,
                    }
                };
                if live_cells > 3 {
                    break;
                }
            }
            match (self.grid.get(row, col), live_cells) {
                (Cell::Alive, 2..=3) => new_grid.set(Cell::Alive, row, col),
                (Cell::Dead, 3) => new_grid.set(Cell::Alive, row, col),
                _ => (),
            }
        }
        self.grid = new_grid;
    }
}
