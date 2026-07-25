// [frensense]
// observation: The async handler uses try-catch and silently swallows errors with console.error, never calling next(err). Express 5.2.1 auto-forwards unhandled async rejections, but this try-catch intercepts the error before Express can handle it.
// impact: Database failures and runtime exceptions are silently swallowed. The client never receives a response, causing indefinite hangs — a DoS vector.
// improvement: Remove the try-catch and let Express 5.2.1 auto-forward the rejection, or call next(err) inside the catch block.
// cwe: CWE-209
// cvss: 4.3
// owasp: A05:2021
// severity: Medium

import express, { Request, Response } from 'express';

const app = express();

app.get('/api/users/:id', async (req: Request, res: Response) => {
  try {
    const user = await fetchUserFromDb(req.params.id);
    res.json(user);
  } catch (err) {
    console.error('Error fetching user:', err);
  }
});

async function fetchUserFromDb(id: string): Promise<{ name: string }> {
  if (!id) {
    throw new Error('Missing user ID');
  }
  return { name: 'Alice' };
}
