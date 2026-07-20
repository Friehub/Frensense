// SAFE alternative: feature-flag gating
function isDebugAllowed(req: Request): boolean {
  return process.env.NODE_ENV === 'development' || req.headers['x-debug-key'] === process.env.DEBUG_KEY;
}

app.get('/debug/env', async (req, res) => {
  if (!isDebugAllowed(req)) {
    return res.status(404).json({ error: 'Not found' });
  }
  const safe = { nodeEnv: process.env.NODE_ENV, appVersion: process.env.APP_VERSION };
  res.json(safe);
});
