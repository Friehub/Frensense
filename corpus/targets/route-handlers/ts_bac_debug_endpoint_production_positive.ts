// [frensense]
// observation: Debug, health, or diagnostic endpoints (e.g., /debug, /info, /env, /metrics, /graphql) are accessible in production without authentication.
// impact: An attacker can extract sensitive information about the infrastructure, environment variables, database schema, and internal configuration from debug endpoints.
// improvement: Disable debug endpoints in production, or protect them with strong authentication (admin-only, VPN access).

import express from 'express';

const app = express();

app.get('/debug/env', (req, res) => {
  res.json(process.env);
});

app.get('/debug/config', (req, res) => {
  res.json({ dbHost: 'prod-db.internal', redisHost: 'redis.internal', apiKeys: process.env });
});

app.get('/info', (req, res) => {
  res.json({ version: '1.0.0', nodeVersion: process.version, platform: process.platform, memory: process.memoryUsage() });
});
