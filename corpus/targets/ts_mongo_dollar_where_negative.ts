// SAFE: $where is not used; filtering is done with safe query operators

import { MongoClient } from 'mongodb';

const client = new MongoClient('mongodb://localhost:27017');

export async function findUsers(age: number, status: string) {
  const db = client.db('app');
  return db.collection('users').find({
    age: { $gte: age },
    status
  }).toArray();
}
