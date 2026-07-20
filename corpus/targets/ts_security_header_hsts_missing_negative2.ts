// SAFE alternative: set on reverse proxy level
import express from 'express';

const app = express();

// For Express behind nginx/cloudflare — ensure HSTS is set at edge
app.use((req, res, next) => {
  if (req.secure) {
    res.setHeader('Strict-Transport-Security', 'max-age=63072000; includeSubDomains');
  }
  next();
});
