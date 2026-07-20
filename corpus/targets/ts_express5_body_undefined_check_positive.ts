// [frensense]
// observation: The handler accesses req.body.name without first checking if req.body is undefined. In Express 5.2.1, req.body is undefined (not {}) when no body parser has been registered or when the Content-Type is not parseable.
// impact: Accessing a property on undefined throws TypeError at runtime, crashing the process and returning a 500 to the client. This can be triggered by sending an unparseable request body.
// improvement: Check req.body for undefined before accessing its properties, and register a body parser middleware.

import express, { Request, Response } from 'express';

const app = express();

app.post('/api/users', (req: Request, res: Response) => {
  const name: string = req.body.name;
  res.json({ message: `Created user ${name}` });
});
