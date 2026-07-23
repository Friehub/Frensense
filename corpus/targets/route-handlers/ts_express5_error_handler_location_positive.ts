// [frensense]
// observation: The error-handling middleware (err, req, res, next) is registered BEFORE route handlers. Express 5.2.1 only invokes error handlers when next(err) is called — but this handler is reached by normal request flow before routes, and since there is no error, it passes through to next() and never catches downstream errors.
// impact: Errors thrown or passed via next(err) in route handlers are never caught by this error handler. Express 5.2.1's default error response is sent instead, which may include stack traces in non-production mode, leaking internal paths and application logic.
// improvement: Move the error-handling middleware to be the LAST middleware, after all route registrations.

import express, { Request, Response, NextFunction } from 'express';

const app = express();

app.use((err: Error, req: Request, res: Response, next: NextFunction) => {
  console.error(err.stack);
  res.status(500).json({ error: 'Something went wrong' });
});

app.get('/api/users/:id', async (req: Request, res: Response) => {
  const user = await fetchUserFromDb(req.params.id);
  res.json(user);
});

async function fetchUserFromDb(id: string): Promise<{ name: string }> {
  if (!id) {
    throw new Error('Invalid user ID');
  }
  return { name: 'Alice' };
}
