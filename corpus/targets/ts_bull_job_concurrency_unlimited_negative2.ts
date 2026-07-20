// SAFE: Uses a limiter to control processing rate in addition to concurrency

import { Worker } from 'bullmq';

const worker = new Worker('email', async job => {
  await sendEmail(job.data.to, job.data.body);
}, {
  connection: { host: 'localhost' },
  concurrency: 10,
  limiter: { max: 50, duration: 1000 },
});
