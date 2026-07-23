// SAFE: set strict Referrer-Policy
import express from 'express';

const app = express();
app.use((req, res, next) => {
  res.setHeader('Referrer-Policy', 'strict-origin-when-cross-origin');
  next();
});
