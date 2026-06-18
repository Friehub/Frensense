// Vulnerable: Detected a channel guarded with a mutex. Channels already have an internal mutex, so this is unnecessary. Remove the mutex. See https://hackmongo.com/page/golang-antipatterns/#guarded-channel for more information.
// Pattern: {'pattern': '$MUX.Lock()\n$VALUE <- $CHANNEL\n$MUX.Unlock()\n'} | {'pattern': '$MUX.Lock()\n$VALUE = <- $CHANNEL\n$MUX.Unlock()\n'}
function vulnerable() {
  // TODO: implement pattern match
}
