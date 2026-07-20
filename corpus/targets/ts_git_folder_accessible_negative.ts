// SAFE: block dotfiles or serve from dist directory
import express from 'express';

const app = express();
app.use(express.static('dist', {
  dotfiles: 'deny',
}));

// SAFE: or exclude .git explicitly
app.use(express.static('public', {
  dotfiles: 'deny',
  index: false,
}));
