// SAFE: validate email format
import { z } from 'zod';

const emailSchema = z.string().email();

app.post('/api/register', async (req, res) => {
  const result = emailSchema.safeParse(req.body.email);
  if (!result.success) {
    return res.status(400).json({ error: 'Invalid email format' });
  }
  const email = result.data;
  const user = await db.createUser({ email, passwordHash: await bcrypt.hash(req.body.password, 12) });
  await sendWelcomeEmail(email);
  res.json({ id: user.id });
});

app.post('/api/subscribe', async (req, res) => {
  const result = emailSchema.safeParse(req.body.email);
  if (!result.success) return res.status(400).json({ error: 'Invalid email' });
  await db.query('INSERT INTO newsletter (email) VALUES ($1)', [result.data]);
  res.json({ status: 'subscribed' });
});
