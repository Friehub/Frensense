// [frensense]
// observation: Debug or development-only endpoints (e.g., /debug, /env, /config, /health/detailed) are accessible in production environments.
// impact: Attackers can discover internal configuration details, environment variables (including secrets), dependency versions, and infrastructure layout. This dramatically reduces the effort needed for targeted exploitation.
// improvement: Guard debug endpoints behind environment checks, authentication, or remove them entirely from production builds.

app.get('/debug/env', (req, res) => {
  // VULNERABLE: exposes all environment variables including secrets
  res.json(process.env);
});

app.get('/debug/config', (req, res) => {
  // VULNERABLE: exposes internal configuration
  res.json({
    dbHost: config.database.host,
    dbName: config.database.name,
    redisUrl: config.redis.url,
    awsRegion: config.aws.region,
    featureFlags: config.features,
  });
});

app.get('/debug/headers', (req, res) => {
  // VULNERABLE: leaks request headers (including auth tokens from upstream proxies)
  res.json(req.headers);
});
