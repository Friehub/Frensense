// SAFE: Barrier wait without timeout; all threads must arrive together.
use std::sync::{Barrier, Arc};

pub fn sync_and_process(barrier: Arc<Barrier>, data: &mut [u8]) {
    barrier.wait();
    data.iter_mut().for_each(|b| *b = b.wrapping_add(1));
}
