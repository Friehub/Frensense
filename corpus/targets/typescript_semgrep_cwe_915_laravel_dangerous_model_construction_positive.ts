// Vulnerable: Setting `$guarded` to an empty array allows mass assignment to every property in a Laravel model. This explicitly overrides Eloquent's safe-by-default mass assignment protections.
// Pattern: $guarded = [];
function vulnerable() {
  // TODO: implement pattern match
}
