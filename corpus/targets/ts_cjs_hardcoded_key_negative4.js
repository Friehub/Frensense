// SAFE: API key from config file with fallback to env var

var config = require('./config');

function resolveSecret(name) {
  return config.get(name) || process.env[name.toUpperCase()];
}

var API_SECRET = resolveSecret('apiSecret');
var JWT_SECRET = resolveSecret('jwtSecret');

function authenticate(req, res) {
  res.json({ authenticated: true });
}

function getStatus(req, res) {
  res.json({ status: 'ok' });
}

module.exports = function(app) {
  app.get('/api/auth', authenticate);
  app.get('/api/status', getStatus);
};
