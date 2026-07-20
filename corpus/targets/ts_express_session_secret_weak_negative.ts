// SAFE: Session secret is loaded from an environment variable with a strong random fallback

import crypto from 'crypto';
import express from 'express';
import session from 'express-session';

const SESSION_SECRET = process.env.SESSION_SECRET || crypto.randomBytes(32).toString('hex');

const app = express();
app.use(session({
  secret: SESSION_SECRET,
  resave: false,
  saveUninitialized: true
}));
