// SAFE: Uses method-based routing with explicit allowlist
import express from 'express';

const app = express();

export function requireMethod(...methods: string[]): express.Handler {
  return (req, res, next) => {
    if (!methods.includes(req.method)) return res.status(405).json({ error: 'Method not allowed' });
    next();
  };
}

app.all('/api/delete-account', requireMethod('DELETE'), async (req, res) => {
  await db.prepare('DELETE FROM users WHERE id = ?').bind(req.session.userId).run();
  res.json({ deleted: true });
});

app.all('/api/transfer-funds', requireMethod('POST'), async (req, res) => {
  const { toAccount, amount } = req.body;
  await db.prepare('UPDATE accounts SET balance = balance - ? WHERE user_id = ?').bind(amount, req.session.userId).run();
  await db.prepare('UPDATE accounts SET balance = balance + ? WHERE id = ?').bind(amount, toAccount).run();
  res.json({ transferred: true });
});
