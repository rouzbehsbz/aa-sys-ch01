use std::{sync::Arc, thread};

use worker::Worker;
use world::World;

mod worker;
mod world;

const WORLD_SIZE: usize = 7;
const BROKEN_CELLS_COUNT: usize = 3;
const WORKERS_COUNT: usize = 4;

fn main() {
    let world = Arc::new(World::new(WORLD_SIZE, WORKERS_COUNT));
    let mut threads = Vec::with_capacity(WORKERS_COUNT);

    world.generate_broken_cells(BROKEN_CELLS_COUNT);

    for id in 0..WORKERS_COUNT {
        let world = world.clone();

        let thread = thread::spawn(move || {
            let mut worker = Worker::new(id, world, BROKEN_CELLS_COUNT, WORLD_SIZE);

            worker.process_until_work_is_done();

            worker.yell_progress();
        });

        threads.push(thread)
    }

    for thread in threads {
        thread.join().unwrap()
    }
}
