// SAFE: Uses a middleware pattern with both user-level and endpoint-level rate limiting

const rateLimitStore = new Map<string, { count: number; resetAt: number }>();

function checkRateLimit(key: string, maxRequests: number, windowMs: number): boolean {
  const now = Date.now();
  const entry = rateLimitStore.get(key);

  if (!entry || now > entry.resetAt) {
    rateLimitStore.set(key, { count: 1, resetAt: now + windowMs });
    return true;
  }

  if (entry.count >= maxRequests) return false;
  entry.count++;
  return true;
}

export async function apiHandler(userId: string, endpoint: string) {
  if (!checkRateLimit(`global:${userId}`, 500, 60000)) {
    throw new Error('Global rate limit exceeded');
  }

  if (!checkRateLimit(`${endpoint}:${userId}`, 100, 60000)) {
    throw new Error('Endpoint rate limit exceeded');
  }

  return handleApiCall(userId, endpoint);
}
