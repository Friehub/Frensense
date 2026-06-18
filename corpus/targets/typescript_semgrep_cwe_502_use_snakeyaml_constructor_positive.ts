// Vulnerable: Used SnakeYAML org.yaml.snakeyaml.Yaml() constructor with no arguments, which is vulnerable to deserialization attacks. Use the one-argument Yaml(...) constructor instead, with SafeConstructor or a custom Constructor as the argument.
// Pattern: $Y = new org.yaml.snakeyaml.Yaml();
...
$Y.load(...);
function vulnerable() {
  // TODO: implement pattern match
}
