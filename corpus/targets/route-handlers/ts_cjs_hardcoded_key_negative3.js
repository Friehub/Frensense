// SAFE: API key loaded from process.env with validation

function validateKey(key) {
  if (!key || key.length < 16) {
    throw new Error('Invalid API key configuration');
  }
  return key;
}

var API_KEY = validateKey(process.env.API_KEY);
var DB_URL = process.env.DATABASE_URL;

function getData(req, res) {
  res.json({ status: 'connected', service: 'api' });
}

module.exports = function(app) {
  app.get('/health', getData);
};
