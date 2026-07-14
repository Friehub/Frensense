// [frensense]
// observation = "The X-Forwarded-For header is trusted to determine the client IP address."
// impact = "An attacker can spoof their IP address by supplying a fake X-Forwarded-For header, bypassing IP-based rate limiting or access controls."
// improvement = "Read the IP from the direct connection socket or the trusted reverse proxy proxying the connection (e.g. request.socket.remoteAddress or Cloudflare CF-Connecting-IP)."

const rateLimiter = rateLimit({
  windowMs: 15 * 60 * 1000,
  max: 100,
  keyGenerator: (req) => {
    // VULNERABLE: trusting X-Forwarded-For which can be set by the client
    return req.headers['x-forwarded-for'] || req.socket.remoteAddress;
  }
});

async function restrictAdminAccess(req: Request) {
  // VULNERABLE: easily spoofed
  const clientIp = req.headers.get('x-forwarded-for');
  if (clientIp !== '10.0.0.1' && clientIp !== '127.0.0.1') {
    return new Response('Forbidden', { status: 403 });
  }
}
