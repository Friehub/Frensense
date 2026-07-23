// SAFE: Queue dashboard separated to admin-only route with session-based auth

import Queue from 'bull';
import Express from 'express';

const app = Express();
const jobQueue = new Queue('email', 'redis://localhost:6379');

function isAdmin(req, res, next) {
  if (req.session?.role !== 'admin') {
    return res.status(403).send('Forbidden');
  }
  next();
}

app.get('/admin/queues/:name/jobs', isAdmin, async (req, res) => {
  const jobs = await jobQueue.getJobs();
  res.json(jobs);
});
