// Vulnerable: Avoid using 'strtok()'. This function directly modifies the first argument buffer, permanently erasing the delimiter character. Use 'strtok_r()' instead.
// Pattern: strtok(...)
function vulnerable() {
  // TODO: implement pattern match
}
