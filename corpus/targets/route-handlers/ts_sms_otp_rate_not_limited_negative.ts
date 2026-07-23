// SAFE: rate limit OTP sending
import { RateLimiter } from './rate-limiter';

const otpRateLimiter = new RateLimiter({
  windowMs: 10 * 60 * 1000, // 10 minutes
  max: 3, // 3 OTPs per window per phone
});

app.post('/api/send-otp', async (req, res) => {
  const { phoneNumber } = req.body;

  // SAFE: rate limit per phone number
  const allowed = await otpRateLimiter.check(phoneNumber);
  if (!allowed) {
    return res.status(429).json({ error: 'Too many OTP requests. Please wait.' });
  }

  const otp = generateOTP();
  await db.storeOTP(phoneNumber, otp);
  await smsClient.send(phoneNumber, `Your code is: ${otp}`);
  res.json({ status: 'sent' });
});
