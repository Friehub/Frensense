// SAFE: Job has a finite retry limit and an error handler that moves failed jobs to a dead-letter queue

import { Worker, Queue } from 'bullmq';

const emailQueue = new Queue('email', { connection: { host: 'localhost' } });

const worker = new Worker('email', async job => {
  await sendEmail(job.data.to, job.data.body);
}, {
  connection: { host: 'localhost' },
  attempts: 5,
  backoff: { type: 'exponential', delay: 1000 },
  removeOnFail: { count: 100 },
});

worker.on('failed', async (job, err) => {
  if (job.attemptsMade >= job.opts.attempts) {
    await deadLetterQueue.add('email-failed', job.data);
  }
});
