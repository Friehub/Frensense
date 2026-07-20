// [frensense]
// observation: Email address field accepted without format validation. Any string is stored as an email address.
// impact: Invalid email addresses cause delivery failures, bounces, and spam complaints. Attackers can inject SQL, NoSQL, or command injection payloads disguised as an email field.
// improvement: Validate email format using a regex, validator library (zod, express-validator), or HTML5 email input type validation server-side.

app.post('/api/register', async (req, res) => {
  // VULNERABLE: email not validated
  const { email, password } = req.body;
  const user = await db.createUser({ email, passwordHash: await bcrypt.hash(password, 12) });
  await sendWelcomeEmail(email);
  res.json({ id: user.id });
});

app.post('/api/subscribe', async (req, res) => {
  // VULNERABLE: any string accepted as email
  const { email } = req.body;
  await db.query('INSERT INTO newsletter (email) VALUES ($1)', [email]);
  res.json({ status: 'subscribed' });
});
