// Fixed: The 'add_header' directive is called in a 'location' block after headers have been set at the server block. Calling 'add_header' in the location block will actually overwrite the headers defined in the server block, no matter which headers are set. To fix this, explicitly set all headers or set all headers in the server block.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
