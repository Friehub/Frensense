// [frensense]
// observation: Date field for a past event (birth date, order date, transaction date) accepts future dates.
// impact: Users can set future birth dates, causing age calculation errors. Order records show future dates, breaking reports and analytics. Backend scheduling logic can malfunction with dates far in the future.
// improvement: Validate that dates for past events are not in the future. Compare against current date or a reasonable maximum.

app.post('/api/profile', async (req, res) => {
  // VULNERABLE: birth date can be in the future
  const { birthDate } = req.body;
  await db.query('UPDATE users SET birth_date = $1 WHERE id = $2', [birthDate, req.user.id]);
  res.json({ status: 'ok' });
});

app.post('/api/transactions', async (req, res) => {
  // VULNERABLE: transaction date can be in the future
  const { amount, date } = req.body;
  await db.query('INSERT INTO transactions (user_id, amount, date) VALUES ($1, $2, $3)',
    [req.user.id, amount, date]);
  res.json({ status: 'ok' });
});
