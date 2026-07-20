// [frensense]
// observation: "Worker is configured without a concurrency limit, allowing unlimited simultaneous job processing."
// impact: "The worker can spawn an unbounded number of concurrent operations, overwhelming downstream databases, APIs, or the Redis connection itself."
// improvement: "Set a concurrency limit on the worker appropriate to your infrastructure capacity."

import { Worker } from 'bullmq';

const worker = new Worker('email', async job => {
  await sendEmail(job.data.to, job.data.body);
}, { connection: { host: 'localhost' } });
