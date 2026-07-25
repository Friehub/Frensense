// [frensense]
// observation: A Mongoose find() call passes the entire request body as the query filter, allowing attackers to inject MongoDB operators.
// impact: Attackers can inject $ne, $regex, $where, or other operators to bypass intended query logic.
// improvement: Never pass raw request bodies to Mongoose queries. Whitelist allowed fields or use a schema-based filter.
// cwe: CWE-22
// cvss: 7.5
// owasp: A01:2021
// severity: High

import mongoose from 'mongoose';

const User = mongoose.model('User', new mongoose.Schema({
  name: String,
  email: String,
  role: String
}));

export async function findUsers(body: any) {
  return User.find(body);
}
