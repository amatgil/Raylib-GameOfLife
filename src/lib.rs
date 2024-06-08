use std::ops::{Index, IndexMut, Not};

#[derive(Clone, Debug)]
pub struct Universe {
    /// Flattened grid of Cells
    cells: Vec<Cell>,
    height: usize,
    width: usize,
}

/// Coordinates, stored as a (row, column) tuple
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coord {
    row: usize, 
    col: usize,
}

impl Coord {
    pub fn new(y: usize, x: usize) -> Coord {
        Self { row: y, col: x } 
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Cell {
    fn is_alive(&self) -> bool {
        *self == Cell::Alive
    }
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![Cell::Dead; width*height];
        Self { cells, height, width }
    }

    pub fn set_dimensions(&mut self, new_dims: Coord) {
        let old = self.clone();
        let mut new = Self {
            cells: vec![Cell::Dead; new_dims.row*new_dims.col],
            height: new_dims.row,
            width: new_dims.col,
        };

        for old_y in 0..(old.height.min(new.height)) {
            for old_x in 0..(old.width.min(new.width)) {
                let coords = Coord::new(old_y, old_x);
                new[coords] = old[coords];
            }
        }

        *self = new;
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn render(&self) -> String {
        self.to_string()
    }
    pub fn toggle_pixel(&mut self, c: Coord) {
        self[c] = !self[c];
    }
    pub fn set_pixel(&mut self, c: Coord, val: Cell) {
        self[c] = val
    }

    pub fn tick(&mut self) {
        self.cells = (0..self.width).zip(0..self.height).map(|(x, y)| {
            match (self[Coord::new(x, y)], self.alive_neighbor_count(x, y)) {
                (Cell::Alive, x) if x < 2           => Cell::Dead,
                (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                (Cell::Alive, x) if x > 3           => Cell::Dead,
                (Cell::Dead, 3)                     => Cell::Alive,
                (current, _)                        => current,
            }
        }).collect();
    }

    fn alive_neighbor_count(&self, x: usize, y: usize) -> u8 {
        let mut cnt = 0;

        for dy in [-1, 0, 1] {
            for dx in [-1, 0, 1] {
                if dx == 0 && dy == 0 { continue; }
                let new_x = (x as i32 + dx).rem_euclid(self.width as i32) as usize;
                let new_y = (y as i32 + dy).rem_euclid(self.height as i32) as usize;

                if self[Coord::new(new_y, new_x)].is_alive() { cnt += 1; }
            }
        }

        cnt
    }

    fn coord_to_idx(&self, c: Coord) -> usize {
        c.col + self.width * c.row
    }

    fn idx_to_coords(&self, i: usize) -> Coord {
        Coord {
            row: i / self.width,
            col: i % self.width,
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Cell::Alive => '◼',
            Cell::Dead => '◻',
        };
        write!(f, "{}", symbol)
    }
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sortida = String::new();
        for idx in 0..self.width * self.height {
            sortida.push_str(&self[idx].to_string());

            if idx % self.width == self.width - 1 { sortida.push('\n') }
        }
        write!(f, "{}", sortida)?;
        Ok(())
    }
}

impl Index<usize> for Universe {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl IndexMut<usize> for Universe {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}
impl Index<Coord> for Universe {
    type Output = Cell;

    fn index(&self, index: Coord) -> &Self::Output {
        let idx = self.coord_to_idx(index);
        &self.cells[idx]
    }
}

impl IndexMut<Coord> for Universe {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        let idx = self.coord_to_idx(index);
        &mut self.cells[idx]
    }
}

impl Not for Cell {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Cell::Dead  => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
}
