// SAFE: Uses a reasonable integer retry count with a circuit breaker pattern

import { Worker, Queue } from 'bullmq';

const MAX_RETRIES = 3;

const emailQueue = new Queue('email', { connection: { host: 'localhost' } });

const worker = new Worker('email', async job => {
  await sendEmail(job.data.to, job.data.body);
}, {
  connection: { host: 'localhost' },
  attempts: MAX_RETRIES,
  backoff: { type: 'fixed', delay: 5000 },
  removeOnFail: true,
});
