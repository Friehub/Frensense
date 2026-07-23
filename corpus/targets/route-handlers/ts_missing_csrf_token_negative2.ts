// SAFE: Uses csurf middleware for automatic CSRF token validation
import csrf from "csurf";

const csrfProtection = csrf({ cookie: true });
app.use(csrfProtection);

app.post('/api/settings/update', async (req, res) => {
  const sessionId = req.cookies['session_id'];
  const user = await validateSession(sessionId);
  if (!user) return res.status(401).send('Unauthorized');
  await db.updateSettings(user.id, req.body.settings);
  res.send('Success');
});
