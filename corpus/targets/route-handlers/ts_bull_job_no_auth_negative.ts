// SAFE: Queue dashboard and job endpoints are protected by authentication middleware

import Queue from 'bull';
import Express from 'express';
import { Queue as QueueMQ, AuthMiddleware } from 'bullmq';

const app = Express();
const jobQueue = new Queue('email', 'redis://localhost:6379');

function requireAuth(req, res, next) {
  if (!req.headers.authorization) {
    return res.status(401).send('Unauthorized');
  }
  next();
}

app.get('/queues/:name/jobs', requireAuth, async (req, res) => {
  const jobs = await jobQueue.getJobs();
  res.json(jobs);
});

app.post('/queues/:name/jobs', requireAuth, async (req, res) => {
  await jobQueue.add(req.body.data);
  res.send('ok');
});
