// SAFE: Uses atomic INCR command to prevent race conditions

import { Ratelimit } from '@upstash/ratelimit';
import { Redis } from '@upstash/redis';

const redis = new Redis({ url: process.env.UPSTASH_REDIS_URL, token: process.env.UPSTASH_REDIS_TOKEN });

class AtomicRatelimit {
  private prefix: string;
  private max: number;
  private window: number;

  constructor(prefix: string, max: number, window: number) {
    this.prefix = prefix;
    this.max = max;
    this.window = window;
  }

  async limit(identifier: string): Promise<{ success: boolean; remaining: number }> {
    const key = `${this.prefix}:${identifier}`;
    const count = await redis.incr(key);

    if (count === 1) {
      await redis.expire(key, this.window);
    }

    if (count > this.max) {
      return { success: false, remaining: 0 };
    }

    return { success: true, remaining: this.max - count };
  }
}

const ratelimit = new AtomicRatelimit('api', 10, 60);

async function handler(req: Request) {
  const result = await ratelimit.limit(req.headers.get('x-forwarded-for') ?? 'unknown');
  if (!result.success) {
    return new Response('Too Many Requests', { status: 429 });
  }
  return new Response('OK');
}
