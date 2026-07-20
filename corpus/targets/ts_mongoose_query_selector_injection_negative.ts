// SAFE: Only whitelisted fields are extracted from the request body for the query filter

import mongoose from 'mongoose';

const User = mongoose.model('User', new mongoose.Schema({
  name: String,
  email: String,
  role: String
}));

const ALLOWED_FILTER_FIELDS = ['name', 'email', 'role'];

export async function findUsers(body: any) {
  const filter: Record<string, any> = {};
  for (const field of ALLOWED_FILTER_FIELDS) {
    if (body[field] !== undefined) {
      filter[field] = body[field];
    }
  }
  return User.find(filter);
}
