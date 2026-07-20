// [frensense]
// observation: A struct holds a self-referential pointer (a field points into another field of the same struct) but is not pinned. When the struct is moved (e.g., passed by value, swapped, or reallocated in a Vec), the pointer becomes dangling.
// impact: After a move, dereferencing the self-referential pointer reads garbage or accesses freed memory, leading to undefined behavior, crashes, or exploitable memory corruption.
// improvement: Ensure the struct is `!Unpin` and only accessed through `Pin<&mut Self>`, or use a crate like `ouroboros` or `self_cell` for safe self-referential patterns.

pub struct BufferView<'data> {
    buf: Vec<u8>,
    view: &'data [u8],
}

impl<'data> BufferView<'data> {
    pub fn new() -> Self {
        let mut buf = vec![0u8; 1024];
        let view = &buf[..];
        BufferView { buf, view }
    }

    pub fn get_view(&self) -> &[u8] {
        self.view
    }
}
