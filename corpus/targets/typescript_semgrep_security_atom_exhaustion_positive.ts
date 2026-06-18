// Vulnerable: Atom values are appended to a global table but never removed. If input is user-controlled, dynamic instantiations such as `String.to_atom` or `List.to_atom` can lead to possible memory leaks. Instead, use `String.to_existing_atom` or `List.to_existing_atom`.
// Pattern: $MODULE.to_atom($STRING)
function vulnerable() {
  // TODO: implement pattern match
}
