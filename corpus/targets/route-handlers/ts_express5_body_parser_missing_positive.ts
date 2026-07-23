// [frensense]
// observation: The POST handler accesses req.body.email without registering a body parser middleware. In Express 5.2.1, req.body is undefined when no body parser is registered (unlike Express 4 which defaulted to {}).
// impact: Accessing req.body.email throws TypeError: Cannot read properties of undefined, crashing the handler. Every POST/PUT/PATCH endpoint silently fails — no form data or JSON is ever parsed.
// improvement: Register express.json() and/or express.urlencoded() at the application level before route handlers.

import express, { Request, Response } from 'express';

const app = express();

app.post('/api/login', (req: Request, res: Response) => {
  const email = req.body.email;
  res.json({ email });
});
