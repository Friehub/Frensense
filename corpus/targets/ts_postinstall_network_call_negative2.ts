// SAFE alternative: require explicit opt-in with env var
{
  "name": "safe-package",
  "scripts": {
    "postinstall": "node -e \"if (process.env.ALLOW_POSTINSTALL === 'true') require('./scripts/setup')\""
  }
}
