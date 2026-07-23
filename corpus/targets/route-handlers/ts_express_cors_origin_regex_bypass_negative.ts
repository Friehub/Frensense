// SAFE: Use an explicit origin whitelist array instead of a regex to prevent bypass via subdomain tricks.

import express from 'express';
import cors from 'cors';

const app = express();
const allowedOrigins = ['https://example.com', 'https://app.example.com'];
const corsOptions = {
  origin: (origin: string | undefined, cb: (err: Error | null, allow?: boolean) => void) => {
    if (!origin || allowedOrigins.includes(origin)) {
      cb(null, true);
    } else {
      cb(new Error('Not allowed by CORS'));
    }
  },
  credentials: true,
};
app.use(cors(corsOptions));

app.get('/api/user/profile', (req, res) => {
  res.json({ email: 'alice@example.com', ssn: '123-45-6789' });
});
