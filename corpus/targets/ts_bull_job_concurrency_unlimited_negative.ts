// SAFE: Worker has an explicit concurrency limit

import { Worker } from 'bullmq';

const worker = new Worker('email', async job => {
  await sendEmail(job.data.to, job.data.body);
}, {
  connection: { host: 'localhost' },
  concurrency: 5,
});
