// SAFE alternative: helmet handles version header removal
import helmet from 'helmet';
import express from 'express';

const app = express();
app.use(helmet());
app.disable('x-powered-by');
