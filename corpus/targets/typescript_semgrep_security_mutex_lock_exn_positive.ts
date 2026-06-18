// Vulnerable: Raw `Mutex.lock`/`Mutex.unlock` is not exception-safe. If an exception is raised between lock and unlock, the mutex will never be released, causing a deadlock. Replace with `Mutex.protect $MUTEX (fun () -> ...)` which guarantees the mutex is released even when exceptions are raised.
// Pattern: Mutex.lock $MUTEX
function vulnerable() {
  // TODO: implement pattern match
}
