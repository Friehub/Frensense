use std::sync::{Barrier, Arc};

pub fn sync_workers(barrier: Arc<Barrier>) {
    barrier.wait();
}
