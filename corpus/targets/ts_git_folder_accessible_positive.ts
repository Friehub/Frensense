// [frensense]
// observation: Static file server serves the .git directory, exposing the entire repository history including commits, branches, and secrets that were ever committed.
// impact: Anyone can fetch /.git/HEAD, /.git/config, and enumerate objects to reconstruct the full source code, commit messages, and any credentials or API keys that were ever committed (even if later removed).
// improvement: Configure the static server to deny access to dotfiles (.git, .env, .hg), or serve files from a build output directory that excludes them.

import express from 'express';

const app = express();
// VULNERABLE: serves everything in 'public/', including .git/
app.use(express.static('public'));
