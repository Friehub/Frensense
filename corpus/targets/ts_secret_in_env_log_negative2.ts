// SAFE: Startup logging uses a structured approach that explicitly includes only safe, non-secret keys

const SAFE_CONFIG_KEYS = ['NODE_ENV', 'PORT', 'LOG_LEVEL', 'REDIS_HOST'];

function logConfig() {
  const safeConfig: Record<string, string | undefined> = {};
  for (const key of SAFE_CONFIG_KEYS) {
    if (process.env[key]) {
      safeConfig[key] = process.env[key];
    }
  }
  console.log('App config:', safeConfig);
  console.log('API_KEY present:', !!process.env.API_KEY);
}
