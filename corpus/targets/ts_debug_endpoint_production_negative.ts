// SAFE: debug endpoints guarded by environment check
const isDev = process.env.NODE_ENV === 'development' || process.env.ENVIRONMENT === 'development';

if (isDev) {
  app.get('/debug/env', (req, res) => {
    const safe = Object.fromEntries(
      Object.entries(process.env).filter(([k]) => !k.toLowerCase().includes('secret') && !k.toLowerCase().includes('key'))
    );
    res.json(safe);
  });
}

// SAFE: config endpoint removed in production
function registerDebugRoutes(app: Express, config: Config) {
  if (process.env.NODE_ENV !== 'production') {
    app.get('/debug/config', (req, res) => {
      res.json({ environment: process.env.NODE_ENV, version: config.appVersion });
    });
  }
}
