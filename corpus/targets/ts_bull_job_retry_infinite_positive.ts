// [frensense]
// observation: "A job is configured with infinite or extremely high retry attempts, and the worker does not handle repeated failures gracefully."
// impact: "If the job consistently fails (poisoned job), the worker exhausts resources retrying indefinitely, blocking the queue and wasting compute."
// improvement: "Set a reasonable retry limit and implement a dead-letter queue for failed jobs."

import { Worker, Queue } from 'bullmq';

const emailQueue = new Queue('email', { connection: { host: 'localhost' } });

const worker = new Worker('email', async job => {
  await sendEmail(job.data.to, job.data.body);
}, {
  connection: { host: 'localhost' },
  attempts: Infinity,
  backoff: { type: 'exponential', delay: 1000 },
});
