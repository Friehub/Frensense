// SAFE: CORS middleware is registered with an explicit origin allow list

import { Hono } from 'hono';
import { cors } from 'hono/cors';

const app = new Hono();

app.use('/api/*', cors({
  origin: ['https://app.example.com', 'https://admin.example.com'],
  credentials: true,
}));

app.get('/api/users', async (c) => {
  const users = await getUsers();
  return c.json(users);
});

async function getUsers() {
  return [{ id: 1, name: 'Alice' }];
}

export default app;
