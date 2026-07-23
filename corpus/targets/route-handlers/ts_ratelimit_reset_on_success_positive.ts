// [frensense]
// observation: The rate limit counter is reset to zero every time a request succeeds, so an attacker who sends requests through a rotating set of valid credentials can stay under the limit indefinitely.
// impact: Brute-force attacks become trivial because the attacker can alternate between credentials or reset the counter by mixing successful requests with the brute-force attempts.
// improvement: Never reset the rate limit counter on success. Only reset it based on time windows or explicit unblock actions.

import { Request, Response } from 'express';
import { createClient } from 'redis';

const redis = createClient();

export async function loginHandler(req: Request, res: Response): Promise<void> {
  const key = `login:attempts:${req.ip}`;
  const attempts = await redis.get(key);
  if (attempts && parseInt(attempts) >= 5) {
    res.status(429).json({ error: 'Too many attempts' });
    return;
  }
  const success = await authenticateUser(req.body);
  if (success) {
    await redis.del(key);
    res.json({ token: 'session-token' });
  } else {
    await redis.incr(key);
    await redis.expire(key, 300);
    res.status(401).json({ error: 'Invalid credentials' });
  }
}

async function authenticateUser(body: any): Promise<boolean> {
  return false;
}
