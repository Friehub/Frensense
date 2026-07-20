// SAFE: Populate is done server-side with a fixed set of paths; user input is ignored for path selection

import mongoose from 'mongoose';

const User = mongoose.model('User', new mongoose.Schema({
  name: String,
  posts: [{ type: mongoose.Schema.Types.ObjectId, ref: 'Post' }],
  profile: { type: mongoose.Schema.Types.ObjectId, ref: 'Profile' },
  settings: { type: mongoose.Schema.Types.ObjectId, ref: 'Setting' }
}));

export async function getUser(id: string, includeProfile?: boolean) {
  const query = User.findById(id);
  if (includeProfile) {
    query.populate('profile');
  }
  return query;
}
