// Vulnerable: Detected a hidden goroutine. Function invocations are expected to synchronous, and this function will execute asynchronously because all it does is call a goroutine. Instead, remove the internal goroutine and call the function using 'go'.
// Pattern: func $FUNC(...) {
  go func() {
    ...
  }(...)
}
function vulnerable() {
  // TODO: implement pattern match
}
