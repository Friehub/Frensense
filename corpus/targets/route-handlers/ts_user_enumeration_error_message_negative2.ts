// SAFE alternative: constant-time login response
app.post('/api/login', async (req, res) => {
  const { email, password } = req.body;
  const user = await db.findUserByEmail(email);
  const hash = user?.passwordHash ?? '$2b$12$placeholder00000000000000000000000000';
  const valid = await bcrypt.compare(password, hash);
  if (!valid || !user) {
    return res.status(401).json({ error: 'Invalid email or password' });
  }
  const token = signToken({ id: user.id });
  res.json({ token });
});
