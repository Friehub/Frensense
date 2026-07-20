// SAFE: set nosniff header on all responses
import express from 'express';
import helmet from 'helmet';

const app = express();
app.use(helmet.noSniff());

// SAFE: or manually
app.use((req, res, next) => {
  res.setHeader('X-Content-Type-Options', 'nosniff');
  next();
});
