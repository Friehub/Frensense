// SAFE: Uses a secret loaded from a secrets manager at startup

import express from 'express';
import session from 'express-session';

async function startServer() {
  const secretResponse = await fetch('https://secrets.example.com/v1/session-secret', {
    headers: { 'Authorization': `Bearer ${process.env.SECRET_MANAGER_TOKEN}` }
  });
  const { secret } = await secretResponse.json();

  const app = express();
  app.use(session({
    secret: [secret, process.env.SESSION_SECRET].filter(Boolean),
    resave: false,
    saveUninitialized: false
  }));
  app.listen(3000);
}
