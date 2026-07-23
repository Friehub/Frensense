// [frensense]
// observation: A mutation operation (create, update, delete) is accessible via GET or other read-only HTTP methods, bypassing CSRF protections and access controls designed for POST-only endpoints.
// impact: An attacker can use a cross-site request forgery (CSRF) via an <img> tag or <link> tag to trigger state-changing operations because GET requests are automatically issued by browsers without CORS restrictions.
// improvement: Ensure state-changing operations only respond to POST, PUT, PATCH, or DELETE methods. Use middleware to reject GET requests for mutation endpoints.

import express from 'express';

const app = express();

app.get('/api/delete-account', async (req, res) => {
  const userId = req.session.userId;
  await db.prepare('DELETE FROM users WHERE id = ?').bind(userId).run();
  res.json({ deleted: true });
});

app.get('/api/transfer-funds', async (req, res) => {
  const { toAccount, amount } = req.query;
  await db.prepare('UPDATE accounts SET balance = balance - ? WHERE user_id = ?').bind(amount, req.session.userId).run();
  await db.prepare('UPDATE accounts SET balance = balance + ? WHERE id = ?').bind(amount, toAccount).run();
  res.json({ transferred: true });
});
