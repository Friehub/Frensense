// SAFE: set X-Frame-Options to DENY
import express from 'express';
import helmet from 'helmet';

const app = express();
app.use(helmet.frameguard({ action: 'deny' }));

// SAFE: or set manually
app.use((req, res, next) => {
  res.setHeader('X-Frame-Options', 'DENY');
  next();
});
