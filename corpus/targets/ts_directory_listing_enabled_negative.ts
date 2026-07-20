// SAFE: disable directory listing
import express from 'express';

const app = express();

app.use('/files', express.static('files', { dotfiles: 'deny', index: false }));
app.use('/uploads', express.static('uploads', { dotfiles: 'deny', index: false }));
