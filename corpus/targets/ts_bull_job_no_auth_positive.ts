// [frensense]
// observation: "Queue dashboard or job processing endpoints exposed without authentication middleware."
// impact: "An attacker can access the admin queue dashboard, view job data including sensitive payloads, add/remove jobs, or manipulate queue state."
// improvement: "Add authentication middleware to all queue dashboard routes and job processing endpoints."

import Queue from 'bull';
import Express from 'express';
import { Queue as QueueMQ } from 'bullmq';

const app = Express();
const jobQueue = new Queue('email', 'redis://localhost:6379');

app.get('/queues/:name/jobs', async (req, res) => {
  const jobs = await jobQueue.getJobs();
  res.json(jobs);
});

app.post('/queues/:name/jobs', async (req, res) => {
  await jobQueue.add(req.body.data);
  res.send('ok');
});

const emailQueue = new QueueMQ('email', { connection: { host: 'localhost' } });

app.get('/admin/queues', async (req, res) => {
  const jobs = await emailQueue.getJobs();
  res.json(jobs);
});
