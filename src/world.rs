use std::{array, collections::HashSet, sync::{Mutex, MutexGuard}};

use rand::Rng;

pub type Coordinates = (usize, usize);
pub type Note = (usize, usize);

pub struct Cell {
    is_broken: bool,
    notes: Vec<Note>
}

impl Cell {
    pub fn new() -> Self {
        Self {
            is_broken: false,
            notes: vec![]
        }
    }
}

pub struct World {
    size: usize,
    cells: Vec<Mutex<Cell>>
}

impl World {
    pub fn new(size: usize) -> Self {
        let cells_count = size*size;
        let mut cells = Vec::with_capacity(cells_count);

        for _ in 0..cells_count {
            cells.push(Mutex::new(Cell::new()));
        }

        Self {
            size,
            cells
        }
    }

    pub fn get_cell_index(&self, cord: Coordinates) -> usize {
        cord.0 * self.size + cord.1
    }

    pub fn get_cell_mut(&self, index: usize) -> MutexGuard<Cell> {
        self.cells[index].lock().unwrap()
    }

    pub fn set_cell_broken_status(&self, cord: Coordinates, is_broken: bool) {
        let index = self.get_cell_index(cord);
        let mut cell = self.get_cell_mut(index);

        cell.is_broken = is_broken
    }

    pub fn generate_broken_cells(&self, count: usize) {
        let mut broken_cells: HashSet<Coordinates> = HashSet::new();
        let mut rng = rand::thread_rng();

        while broken_cells.len() < count {
            let row = rng.gen_range(0..self.size);
            let col = rng.gen_range(0..self.size);

            broken_cells.insert((row, col));
        }

        for cord in broken_cells {
            self.set_cell_broken_status(cord, true)
        }
    }
}