// [frensense]
// observation: A MongoDB $where operator is used with user-controlled input as a JavaScript expression.
// impact: An attacker can inject arbitrary JavaScript code that executes on the database server, leading to NoSQL injection.
// improvement: Avoid using $where entirely. Use $expr or perform filtering in application code instead.

import { MongoClient } from 'mongodb';

const client = new MongoClient('mongodb://localhost:27017');

export async function findUsers(expression: string) {
  const db = client.db('app');
  return db.collection('users').find({ $where: expression }).toArray();
}
