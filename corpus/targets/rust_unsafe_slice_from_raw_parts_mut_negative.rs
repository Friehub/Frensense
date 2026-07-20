pub fn safe_slice(buf: &mut [u8]) -> &mut [u8] {
    &mut buf[..]
}
