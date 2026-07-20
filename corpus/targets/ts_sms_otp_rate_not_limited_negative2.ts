// SAFE alternative: IP + phone rate limiting
import { RateLimiter } from './rate-limiter';

const ipLimiter = new RateLimiter({ windowMs: 60 * 1000, max: 5 });
const phoneLimiter = new RateLimiter({ windowMs: 10 * 60 * 1000, max: 3 });

app.post('/api/send-otp', async (req, res) => {
  const { phoneNumber } = req.body;
  if (!await ipLimiter.check(req.ip)) return res.status(429).json({ error: 'Too many requests' });
  if (!await phoneLimiter.check(phoneNumber)) return res.status(429).json({ error: 'Too many OTPs' });
  // ...
});
