// SAFE: Application-level filtering is used instead of $where database expression

import { MongoClient } from 'mongodb';

const client = new MongoClient('mongodb://localhost:27017');

function evaluateCondition(user: any, expression: string): boolean {
  if (expression === 'isActive') return user.status === 'active';
  if (expression === 'isVip') return user.loyaltyPoints > 1000;
  return false;
}

export async function findUsers(filterExpression: string) {
  const db = client.db('app');
  const allUsers = await db.collection('users').find({}).toArray();
  return allUsers.filter(user => evaluateCondition(user, filterExpression));
}
