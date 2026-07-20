// SAFE: If a regex is needed, anchor it with ^ and $ to prevent partial hostname matches.

import express from 'express';
import cors from 'cors';

const app = express();
const corsOptions = {
  origin: /^https:\/\/([\w-]+\.)*example\.com$/,
  credentials: true,
};
app.use(cors(corsOptions));

app.get('/api/user/profile', (req, res) => {
  res.json({ email: 'alice@example.com', ssn: '123-45-6789' });
});
