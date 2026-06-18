// Vulnerable: Double.Epsilon is defined by .NET as the smallest value that can be added to or subtracted from a zero-value Double. It is unsuitable for equality comparisons of non-zero Double values. Furthermore, the value of Double.Epsilon is framework and processor architecture dependent. Wherever possible, developers should prefer the framework Equals() method over custom equality implementations.
// Pattern: $V1 - $V2
function vulnerable() {
  // TODO: implement pattern match
}
