use std::{
    array,
    collections::HashSet,
    sync::{mpsc::Receiver, Arc, Mutex, MutexGuard},
};

use rand::Rng;

pub type Coordinates = (usize, usize);
pub type Note = (usize, usize);

pub struct Cell {
    is_broken: bool,
    notes: Vec<Note>,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            is_broken: false,
            notes: vec![],
        }
    }
}

pub struct World {
    size: usize,
    cells: Vec<Mutex<Cell>>,
    ready_workers: Mutex<usize>,
}

impl World {
    pub fn new(size: usize) -> Self {
        let cells_count = size * size;
        let mut cells = Vec::with_capacity(cells_count);

        for _ in 0..cells_count {
            cells.push(Mutex::new(Cell::new()));
        }

        Self {
            size,
            cells,
            ready_workers: Mutex::new(0),
        }
    }

    pub fn get_cell_index(&self, cord: Coordinates) -> usize {
        cord.0 * self.size + cord.1
    }

    pub fn get_cell_mut(&self, index: usize) -> MutexGuard<Cell> {
        self.cells[index].lock().unwrap()
    }

    pub fn repair_cell(&self, worker_id: usize, worker_repaired_cells: usize, cord: Coordinates) -> usize {
        let index = self.get_cell_index(cord);
        let mut cell = self.get_cell_mut(index);
        let mut new_repaired_cells = worker_repaired_cells;

        if cell.is_broken {
            cell.is_broken = false;
            new_repaired_cells += 1;
        }

        cell.notes.push((worker_id, new_repaired_cells));

        new_repaired_cells
    }

    pub fn get_repaired_cells_from_notes(&self, cord: Coordinates) -> usize {
        let index = self.get_cell_index(cord);
        let cell = self.get_cell_mut(index);
        let mut all_repaired_cells = 0;

        for (_, cells_repaired) in &cell.notes {
            if *cells_repaired > all_repaired_cells {
                all_repaired_cells = *cells_repaired;
            }
        }

        all_repaired_cells
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
            let index = self.get_cell_index(cord);
            let mut cell = self.get_cell_mut(index);
    
            cell.is_broken = true
        }
    }
}
