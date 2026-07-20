// SAFE: Populate paths are restricted to an allowlist

import mongoose from 'mongoose';

const User = mongoose.model('User', new mongoose.Schema({
  name: String,
  posts: [{ type: mongoose.Schema.Types.ObjectId, ref: 'Post' }],
  profile: { type: mongoose.Schema.Types.ObjectId, ref: 'Profile' }
}));

const ALLOWED_POPULATE = ['posts', 'profile', 'posts.author'];

export async function getUser(id: string, populatePath: string) {
  if (!ALLOWED_POPULATE.includes(populatePath)) {
    throw new Error('Invalid populate path');
  }
  return User.findById(id).populate(populatePath);
}
