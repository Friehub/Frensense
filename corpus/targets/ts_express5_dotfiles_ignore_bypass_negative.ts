// SAFE: Explicitly set dotfiles: 'allow' to serve .well-known files.

import express from 'express';

const app = express();

app.use(express.static('public', { dotfiles: 'allow' }));
