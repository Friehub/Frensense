// SAFE: Helmet is installed and used as the first middleware, setting security headers automatically.

const express = require('express');
const helmet = require('helmet');

const app = express();
app.use(helmet());

app.get('/api/login', function(req, res) {
  res.send('<form action="/login" method="POST"><input name="pw" type="password"><button>Login</button></form>');
});
