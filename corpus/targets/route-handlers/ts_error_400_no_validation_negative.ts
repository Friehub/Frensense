// SAFE: Each validation failure returns a specific, helpful error message.
import { Request, Response } from 'express';

export async function createUser(req: Request, res: Response): Promise<void> {
  const errors: string[] = [];

  if (!req.body.email || typeof req.body.email !== 'string' || !req.body.email.includes('@')) {
    errors.push('email must be a valid email address');
  }
  if (!req.body.password || typeof req.body.password !== 'string' || req.body.password.length < 8) {
    errors.push('password must be at least 8 characters');
  }
  if (!req.body.age || typeof req.body.age !== 'number' || req.body.age < 18) {
    errors.push('age must be a number >= 18');
  }

  if (errors.length > 0) {
    res.status(400).json({ error: 'Validation failed', details: errors });
    return;
  }

  await saveUser(req.body.email, req.body.password, req.body.age);
  res.status(201).json({ ok: true });
}

async function saveUser(email: string, password: string, age: number): Promise<void> {
  console.log('saving', email);
}
