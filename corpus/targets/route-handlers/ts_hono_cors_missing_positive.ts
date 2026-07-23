// [frensense]
// observation: A Hono application has a cross-origin route that returns sensitive data without CORS middleware, so browsers block the response for frontend apps on different origins.
// impact: The endpoint may still be accessed via server-side requests without restriction, but client-side cross-origin requests are either blocked or—if a permissive CORS header is set manually—allow unrestricted access.
// improvement: Register the CORS middleware using app.use('*', cors()) with an explicit origin allow list.

import { Hono } from 'hono';

const app = new Hono();

app.get('/api/users', async (c) => {
  const users = await getUsers();
  return c.json(users);
});

async function getUsers() {
  return [{ id: 1, name: 'Alice' }];
}

export default app;
