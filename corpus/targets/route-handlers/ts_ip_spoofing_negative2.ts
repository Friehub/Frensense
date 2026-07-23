// SAFE: Uses the socket's remoteAddress directly and only falls back to trusted proxy headers
const rateLimiter = rateLimit({
  windowMs: 15 * 60 * 1000,
  max: 100,
  keyGenerator: (req) => {
    return req.ip || req.socket.remoteAddress;
  }
});

async function restrictAdminAccess(req: Request) {
  const clientIp = req.socket.remoteAddress;
  if (clientIp !== '127.0.0.1' && clientIp !== '::1' && !clientIp?.startsWith('10.')) {
    return new Response('Forbidden', { status: 403 });
  }
}
