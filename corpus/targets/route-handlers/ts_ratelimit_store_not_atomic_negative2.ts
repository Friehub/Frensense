// SAFE: Uses Upstash's built-in atomic Ratelimit with sliding window

import { Ratelimit } from '@upstash/ratelimit';
import { Redis } from '@upstash/redis';

const redis = new Redis({ url: process.env.UPSTASH_REDIS_URL, token: process.env.UPSTASH_REDIS_TOKEN });

const ratelimit = new Ratelimit({
  redis,
  limiter: Ratelimit.slidingWindow(10, '60 s'),
  analytics: true,
  prefix: 'ratelimit',
});

async function handler(req: Request) {
  const identifier = req.headers.get('x-forwarded-for') ?? 'unknown';
  const { success, remaining, reset } = await ratelimit.limit(identifier);

  if (!success) {
    return new Response('Too Many Requests', {
      status: 429,
      headers: { 'Retry-After': String(reset) },
    });
  }

  return new Response('OK');
}
