// SAFE alternative: serve from build output, never from repo root
import express from 'express';
import { join } from 'path';

const app = express();
// SAFE: only serving compiled output
app.use(express.static(join(__dirname, '../../client/build')));
