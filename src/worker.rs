use std::{sync::Arc, thread};

use rand::Rng;

use crate::world::{Coordinates, World};

#[derive(Default, Clone, Copy)]
pub enum WorkerStatus {
    Idle,
    Move,
    Repair,
    #[default]
    Wait,
}

pub struct Worker {
    id: usize,
    world: Arc<World>,
    world_size: usize,
    cells_need_to_repair: usize,
    repaired_cells: usize,
    status: WorkerStatus,
    position: Coordinates,
}

impl Worker {
    pub fn new(
        id: usize,
        world: Arc<World>,
        cells_need_to_repair: usize,
        world_size: usize
    ) -> Self {
        let mut rng = rand::thread_rng();
        let start_position_row = rng.gen_range(0..world_size);
        let start_position_col = rng.gen_range(0..world_size);

        Self {
            id,
            world,
            world_size,
            cells_need_to_repair,
            repaired_cells: 0,
            status: WorkerStatus::default(),
            position: (start_position_row, start_position_col),
        }
    }

    pub fn get_status(&self) -> WorkerStatus {
        self.status
    }

    pub fn generate_status(&mut self) {
        let mut rng = rand::thread_rng();

        let status = match rng.gen_range(0..=2) {
            0 => WorkerStatus::Idle,
            1 => WorkerStatus::Move,
            _ => WorkerStatus::Repair,
        };

        self.status = status
    }

    pub fn move_next_position(&mut self) {
        let mut rng = rand::thread_rng();

        let mut is_vertical = rng.gen_bool(0.5);
        let mut is_forward = rng.gen_bool(0.5);

        if self.position.0 == 0 || self.position.0 == self.world_size - 1 {
            is_vertical = true
        }
        else if self.position.1 == 0 || self.position.1 == self.world_size - 1 {
            is_vertical = false
        }

        if self.position.0 == 0 || self.position.1 == 0 {
            is_forward = true
        }
        else if self.position.0 == self.world_size - 1 || self.position.0 == self.world_size - 1 {
            is_forward = false
        }

        match is_vertical {
            true => match is_forward {
                true => {
                    self.position.1 += 1;
                }
                false => {
                    self.position.1 -= 1;
                }
            },
            false => match is_forward {
                true => {
                    self.position.0 += 1;
                }
                false => {
                    self.position.0 -= 1;
                }
            },
        }
    }

    pub fn is_work_done(&self, all_repaired_cells: usize) -> bool {
        self.cells_need_to_repair == all_repaired_cells
    }

    pub fn process_until_work_is_done(&mut self) {
        loop {
            match self.status {
                WorkerStatus::Wait => {
                    
                }
                WorkerStatus::Move => {
                    self.move_next_position();
                    self.status = WorkerStatus::Wait;
                }
                WorkerStatus::Repair => {
                    self.repaired_cells = self.world.repair_cell(self.id, self.repaired_cells, self.position);
                    self.status = WorkerStatus::Wait;
                }
                WorkerStatus::Idle => {
                    self.status = WorkerStatus::Wait;
                }
            }

            let all_repaired_cells = self.world.get_repaired_cells_from_notes(self.position);
            let is_done = self.is_work_done(all_repaired_cells);

            if is_done {
                break;
            }
        }
    }

    pub fn yell_progress(&self) {
        println!("Worker with ID of {}, repaired {} cells.", self.id, self.repaired_cells)
    }
}
