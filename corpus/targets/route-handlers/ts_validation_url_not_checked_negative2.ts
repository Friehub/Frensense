// SAFE alternative: zod URL validation
import { z } from 'zod';

const urlSchema = z.string().url().refine(u => u.startsWith('https://'), { message: 'Only HTTPS URLs allowed' });

app.post('/api/avatar', async (req, res) => {
  const result = urlSchema.safeParse(req.body.imageUrl);
  if (!result.success) return res.status(400).json({ error: 'Invalid image URL' });
  const response = await fetch(result.data);
  // ...
});
