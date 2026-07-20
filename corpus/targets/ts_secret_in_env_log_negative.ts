// SAFE: Secrets are filtered out before logging environment configuration

const SECRET_VARS = ['API_KEY', 'DATABASE_URL', 'REDIS_PASSWORD', 'JWT_SECRET'];

function logConfig() {
  const safeConfig: Record<string, string> = {};
  for (const [key, value] of Object.entries(process.env)) {
    if (value && !SECRET_VARS.includes(key)) {
      safeConfig[key] = value;
    }
  }
  console.log('Starting app with config:', safeConfig);
}
