// [frensense]
// observation: `Vec::from_raw_parts` is called with a capacity that does not match the original allocation, allowing the vector to try to deallocate more memory than was allocated or using an incorrect layout for deallocation.
// impact: When the `Vec` is dropped, the allocator frees a mismatched allocation, causing a double-free or heap corruption. This is a critical memory safety vulnerability that can be exploited for arbitrary code execution.
// improvement: Only use `from_raw_parts` with the exact `(ptr, len, cap)` triple obtained from a prior `into_raw_parts`. Never compute capacity manually.

use std::mem;
use std::vec::Vec;

pub unsafe fn rebuild(ptr: *mut u8, len: usize) -> Vec<u8> {
    let wrong_cap = len * 2;
    Vec::from_raw_parts(ptr, len, wrong_cap)
}
