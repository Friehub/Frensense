// [frensense]
// observation: "Rate limit store operations are not atomic, causing race conditions under concurrent requests."
// impact: "Under high concurrency, two requests can read the same counter value before either writes, allowing an attacker to send up to 2x the intended limit by timing requests together."
// improvement: "Use atomic increment operations (e.g., Redis INCR) or Lua scripts for read-and-increment in a single step."
// cwe: CWE-770
// cvss: 5.3
// owasp: A04:2021
// severity: Medium

import { Ratelimit } from '@upstash/ratelimit';
import { Redis } from '@upstash/redis';

const redis = new Redis({ url: process.env.UPSTASH_REDIS_URL, token: process.env.UPSTASH_REDIS_TOKEN });

class NonAtomicRatelimit {
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
    const current = await redis.get<number>(key);
    const count = current ?? 0;

    if (count >= this.max) {
      return { success: false, remaining: 0 };
    }

    await redis.set(key, count + 1, { ex: this.window });
    return { success: true, remaining: this.max - count - 1 };
  }
}

const ratelimit = new NonAtomicRatelimit('api', 10, 60);

async function handler(req: Request) {
  const result = await ratelimit.limit(req.headers.get('x-forwarded-for') ?? 'unknown');
  if (!result.success) {
    return new Response('Too Many Requests', { status: 429 });
  }
  return new Response('OK');
}
