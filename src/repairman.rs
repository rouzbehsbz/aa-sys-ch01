use rand::Rng;

use crate::world::Coordinates;

#[derive(Default, Clone, Copy)]
pub enum RepairmanStatus {
    Idle,
    Move,
    #[default]
    Repair
}

pub struct Repairman {
    cells_need_to_repair: usize,
    cells_repaird: usize,
    status: RepairmanStatus,
    position: Coordinates
}

impl Repairman {
    pub fn new(cells_need_to_repair: usize, position: Coordinates) -> Self {        
        Self {
            cells_need_to_repair,
            cells_repaird: 0,
            status: RepairmanStatus::default(),
            position
        }
    }

    pub fn get_status(&self) -> RepairmanStatus {
        self.status
    }

    pub fn generate_status(&mut self) {
        let mut rng = rand::thread_rng();

        let status = match rng.gen_range(0..=2) {
            0 => RepairmanStatus::Idle,
            1 => RepairmanStatus::Move,
            _ => RepairmanStatus::Repair,
        };

        self.status = status
    }

    pub fn is_work_done(&self) -> bool {
        self.cells_need_to_repair == self.cells_repaird
    }

    pub fn next_move(&mut self) {
        let mut rng = rand::thread_rng();

        let is_vertical = rng.gen_bool(0.5);
        let is_forward = rng.gen_bool(0.5);

        match is_vertical {
            true => {
                match is_forward {
                    true => {
                        self.position.1 += 1;
                    },
                    false => {
                        self.position.1 -= 1;
                    },
                }
            },
            false =>  match is_forward {
                true => {
                    self.position.0 += 1;
                },
                false => {
                    self.position.0 -= 1;
                },
            }
        }
    }
}