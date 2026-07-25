// [frensense]
// observation: Mongoose .populate() receives a path value from user input, allowing access to arbitrary collections.
// impact: An attacker can populate sensitive relationships not intended for public access, leaking related data.
// improvement: Whitelist allowed populate paths and never accept them from user input.
// cwe: CWE-22
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: path_traversal

import mongoose from 'mongoose';

const User = mongoose.model('User', new mongoose.Schema({
  name: String,
  posts: [{ type: mongoose.Schema.Types.ObjectId, ref: 'Post' }],
  profile: { type: mongoose.Schema.Types.ObjectId, ref: 'Profile' }
}));

export async function getUser(id: string, populatePath: string) {
  return User.findById(id).populate(populatePath);
}
