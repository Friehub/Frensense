// SAFE: validate date is not in the future
app.post('/api/profile', async (req, res) => {
  const { birthDate } = req.body;
  const date = new Date(birthDate);
  if (isNaN(date.getTime()) || date > new Date()) {
    return res.status(400).json({ error: 'Birth date cannot be in the future' });
  }
  if (date < new Date('1900-01-01')) {
    return res.status(400).json({ error: 'Invalid birth date' });
  }
  await db.query('UPDATE users SET birth_date = $1 WHERE id = $2', [date.toISOString(), req.user.id]);
  res.json({ status: 'ok' });
});

app.post('/api/transactions', async (req, res) => {
  const { amount, date } = req.body;
  const txDate = new Date(date);
  if (isNaN(txDate.getTime()) || txDate > new Date()) {
    return res.status(400).json({ error: 'Transaction date cannot be in the future' });
  }
  await db.query('INSERT INTO transactions (user_id, amount, date) VALUES ($1, $2, $3)',
    [req.user.id, amount, txDate.toISOString()]);
  res.json({ status: 'ok' });
});
