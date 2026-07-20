// SAFE: Uses Zod schema validation with descriptive error flattening.
import { Request, Response } from 'express';
import { z } from 'zod';

const createUserSchema = z.object({
  email: z.string().email('must be a valid email'),
  password: z.string().min(8, 'password must be at least 8 characters'),
  age: z.number().int().min(18, 'must be at least 18 years old'),
});

export async function createUser(req: Request, res: Response): Promise<void> {
  const parsed = createUserSchema.safeParse(req.body);
  if (!parsed.success) {
    const issues = parsed.error.issues.map(i => ({ field: i.path.join('.'), message: i.message }));
    res.status(400).json({ error: 'Validation failed', details: issues });
    return;
  }

  await saveUser(parsed.data.email, parsed.data.password, parsed.data.age);
  res.status(201).json({ ok: true });
}

async function saveUser(email: string, password: string, age: number): Promise<void> {
  console.log('saving', email);
}
