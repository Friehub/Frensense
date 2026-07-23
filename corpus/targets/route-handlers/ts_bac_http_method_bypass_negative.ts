// SAFE: State-changing operations only respond to POST/DELETE/PUT
import express from 'express';

const app = express();

app.post('/api/delete-account', async (req, res) => {
  const userId = req.session.userId;
  await db.prepare('DELETE FROM users WHERE id = ?').bind(userId).run();
  res.json({ deleted: true });
});

app.post('/api/transfer-funds', async (req, res) => {
  const { toAccount, amount } = req.body;
  await db.prepare('UPDATE accounts SET balance = balance - ? WHERE user_id = ?').bind(amount, req.session.userId).run();
  await db.prepare('UPDATE accounts SET balance = balance + ? WHERE id = ?').bind(amount, toAccount).run();
  res.json({ transferred: true });
});
