// [frensense]
// observation: The Express 5 route handler is an async function that throws without calling next(err). Express 5 does not automatically catch rejected promises — an unhandled promise rejection causes the server to hang silently.
// impact: An attacker can send a request that triggers validation to fail, causing the server to hang indefinitely (DoS via resource exhaustion). The client never receives a response.
// improvement: Wrap async handlers in a try/catch that calls next(err), or use a library like express-async-errors to patch Express 5.

import express, { Request, Response, NextFunction } from 'express';

const app = express();

app.get('/api/users/:id', async (req: Request, res: Response) => {
  const user = await fetchUserFromDb(req.params.id);
  res.json(user);
});

async function fetchUserFromDb(id: string): Promise<{ name: string }> {
  if (!id) {
    throw new Error('Missing user ID');
  }
  return { name: 'Alice' };
}
