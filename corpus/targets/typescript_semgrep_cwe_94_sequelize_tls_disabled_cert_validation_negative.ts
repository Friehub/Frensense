// Fixed: Set "rejectUnauthorized" to false is a convenient way to resolve certificate error. But this method is unsafe because it disables the server certificate verification, making the Node app open to MITM attack. "rejectUnauthorized" option must be alway set to True (default value). With self -signed certificate or custom CA, use "ca" option to define Root Certificate. This rule checks TLS configuration only for Postgresql, MariaDB and MySQL. SQLite is not really concerned by TLS configuration. This rule could be extended for MSSQL, but the dialectOptions is specific for Tedious.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
