// Vulnerable: ASP.NET applications built with `debug` set to true in production may leak debug information to attackers. Debug mode also affects performance and reliability. Set `debug` to `false` or remove it from `<compilation ... />`
// Pattern: <compilation ... debug = "true" ... />
function vulnerable() {
  // TODO: implement pattern match
}
