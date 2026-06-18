// Fixed: 'Integer.toHexString()' strips leading zeroes from each byte if read byte-by-byte. This mistake weakens the hash value computed since it introduces more collisions. Use 'String.format("%02X", ...)' instead.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
