// SAFE: block .map files in production
import express from 'express';

const app = express();
app.use(express.static('dist', {
  dotfiles: 'deny',
  setHeaders: (res, path) => {
    if (path.endsWith('.map')) {
      res.set('Content-Type', 'application/octet-stream');
      res.status(403);
    }
  },
}));
