// [frensense]
// observation: Validation error returns a generic 400 with no message, forcing the client to guess what's wrong. This obscures the validation failure while still leaking that the request was malformed.
// impact: Poor security posture — the "security by obscurity" approach (hiding error details) fails because attackers can still infer valid inputs via timing or trial-and-error, while legitimate users have no guidance.
// improvement: Return descriptive validation errors with field-level details and accepted constraints.
// cwe: CWE-209
// cvss: 4.3
// owasp: A05:2021
// severity: Medium

import { Request, Response } from 'express';

export async function createUser(req: Request, res: Response): Promise<void> {
  const { email, password, age } = req.body;

  if (!email || !password || !age) {
    res.status(400).json({ error: 'Bad request' });
    return;
  }

  if (typeof email !== 'string' || !email.includes('@')) {
    res.status(400).json({ error: 'Bad request' });
    return;
  }

  if (typeof password !== 'string' || password.length < 8) {
    res.status(400).json({ error: 'Bad request' });
    return;
  }

  if (typeof age !== 'number' || age < 18) {
    res.status(400).json({ error: 'Bad request' });
    return;
  }

  await saveUser(email, password, age);
  res.status(201).json({ ok: true });
}

async function saveUser(email: string, password: string, age: number): Promise<void> {
  console.log('saving', email);
}
