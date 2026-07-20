// SAFE: Uses bytemuck for safe cast with size and alignment checks
use bytemuck;

fn cast_bytes_to_u32(bytes: &[u8]) -> &[u32] {
    bytemuck::cast_slice(bytes)
}

fn cast_ref<T: bytemuck::Pod, U: bytemuck::Pod>(val: &T) -> &U {
    bytemuck::cast_ref(val)
}
