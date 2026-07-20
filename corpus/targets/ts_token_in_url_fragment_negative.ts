// SAFE: Token exchanged server-side via code, never exposed in URL fragment
import { NextApiRequest, NextApiResponse } from 'next';

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
  const { code } = req.query;
  const tokenResponse = await fetch('https://oauth.example.com/token', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      code,
      client_id: process.env.CLIENT_ID,
      client_secret: process.env.CLIENT_SECRET,
      redirect_uri: process.env.REDIRECT_URI,
    }),
  });
  const { access_token } = await tokenResponse.json();
  const sessionCookie = await encryptSession(access_token);
  res.setHeader('Set-Cookie', sessionCookie);
  res.redirect('/dashboard');
}
