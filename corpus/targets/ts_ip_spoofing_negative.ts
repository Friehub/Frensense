// SAFE: using trusted connection headers or sockets
const rateLimiter = rateLimit({
  windowMs: 15 * 60 * 1000,
  max: 100,
  keyGenerator: (req) => {
    // SAFE: using the direct socket connection or trusted Cloudflare header
    return req.headers['cf-connecting-ip'] || req.socket.remoteAddress;
  }
});

async function restrictAdminAccess(req: Request) {
  // SAFE: CF-Connecting-IP is stripped from incoming requests by Cloudflare
  // and replaced with the actual client IP, making it safe to trust.
  const clientIp = req.headers.get('cf-connecting-ip');
  if (clientIp !== '10.0.0.1' && clientIp !== '127.0.0.1') {
    return new Response('Forbidden', { status: 403 });
  }
}
