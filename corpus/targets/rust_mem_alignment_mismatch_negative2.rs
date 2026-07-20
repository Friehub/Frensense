// SAFE: Serializes data with proper alignment by using byteorder crate methods instead of raw pointer casts.

use byteorder::{ByteOrder, BigEndian};

fn read_u32_safe(buf: &[u8]) -> u32 {
    BigEndian::read_u32(buf)
}

fn write_u64_safe(buf: &mut [u8]) {
    BigEndian::write_u64(buf, 42);
}
