// [frensense]
// observation: There is no mechanism to revoke access tokens once issued. Tokens remain valid until their natural expiry, even if the device is lost, the user logs out, or a compromise is detected.
// impact: A stolen or leaked token continues to grant access until expiration, which could be hours or days later.
// improvement: Maintain a revocation list or token version number checked on every request.

import { expressjwt } from 'express-jwt';
import express from 'express';

const app = express();

app.use(expressjwt({
  secret: process.env.JWT_SECRET!,
  algorithms: ['HS256'],
}));

export function listPosts(req: express.Request, res: express.Response) {
  res.json({ posts: [] });
}
