// SAFE: validate string length
import { z } from 'zod';

const profileSchema = z.object({
  name: z.string().min(1).max(100),
  bio: z.string().max(500).optional(),
});

app.post('/api/profile', async (req, res) => {
  const result = profileSchema.safeParse(req.body);
  if (!result.success) return res.status(400).json({ errors: result.error.flatten() });
  const { name, bio } = result.data;
  await db.query('UPDATE users SET name = $1, bio = $2 WHERE id = $3', [name, bio, req.user.id]);
  res.json({ status: 'ok' });
});

const commentSchema = z.object({
  postId: z.string().uuid(),
  body: z.string().min(1).max(5000),
});

app.post('/api/comments', async (req, res) => {
  const result = commentSchema.safeParse(req.body);
  if (!result.success) return res.status(400).json({ errors: result.error.flatten() });
  await db.query('INSERT INTO comments (post_id, author_id, body) VALUES ($1, $2, $3)',
    [result.data.postId, req.user.id, result.data.body]);
  res.json({ status: 'ok' });
});
