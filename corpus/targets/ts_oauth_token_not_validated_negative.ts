// SAFE: verify token with provider's introspection endpoint
import { OAuth2Client } from 'google-auth-library';

const googleClient = new OAuth2Client(process.env.GOOGLE_CLIENT_ID);

app.post('/api/auth/google', async (req, res) => {
  const { idToken } = req.body;

  // SAFE: verify token signature and claims
  const ticket = await googleClient.verifyIdToken({
    idToken,
    audience: process.env.GOOGLE_CLIENT_ID,
  });

  const payload = ticket.getPayload();
  const user = await db.findOrCreateUser({ email: payload.email, name: payload.name });
  const token = signToken({ id: user.id });
  res.json({ token });
});

app.post('/api/auth/github', async (req, res) => {
  const { code } = req.body;
  // SAFE: exchange code for token server-side
  const tokenResponse = await fetch('https://github.com/login/oauth/access_token', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', Accept: 'application/json' },
    body: JSON.stringify({
      client_id: process.env.GITHUB_CLIENT_ID,
      client_secret: process.env.GITHUB_CLIENT_SECRET,
      code,
    }),
  }).then(r => r.json());

  const accessToken = tokenResponse.access_token;
  // SAFE: token was issued by GitHub directly
  const userInfo = await fetch('https://api.github.com/user', {
    headers: { Authorization: `Bearer ${accessToken}` },
  }).then(r => r.json());

  const user = await db.findOrCreateUser({ githubId: userInfo.id, name: userInfo.login });
  const token = signToken({ id: user.id });
  res.json({ token });
});
