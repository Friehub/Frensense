// SAFE: Helmet is installed as the first middleware, setting all security headers automatically via the well-audited helmet library.

import express from 'express';
import helmet from 'helmet';

const app = express();
app.use(helmet());

app.get('/api/login', (req, res) => {
  res.send('<form action="/login" method="POST"><input name="pw" type="password"><button>Login</button></form>');
});
