// Vulnerable: If TLS is disabled on server side (Postgresql server), Sequelize establishes connection without TLS and no error will be thrown. To prevent MITN (Man In The Middle) attack, TLS must be enforce by Sequelize. Set "ssl: true" or define settings "ssl: {...}"
// Pattern: {
  host: $HOST,
  database: $DATABASE,
  dialect: $DIALECT
 }
function vulnerable() {
  // TODO: implement pattern match
}
