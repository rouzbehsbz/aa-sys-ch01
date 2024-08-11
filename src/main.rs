use worker::Worker;
use world::World;

mod worker;
mod world;

const WORLD_SIZE: usize = 7;
const BROKEN_CELLS_COUNT: usize = 4;
const WORKERS_COUNT: usize = 4;

fn main() {
    let world = World::new(WORLD_SIZE);
    let workers: Vec<Worker> = Vec::with_capacity(WORKERS_COUNT);

    world.generate_broken_cells(BROKEN_CELLS_COUNT);
}
