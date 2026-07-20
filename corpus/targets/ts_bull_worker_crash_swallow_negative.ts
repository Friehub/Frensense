// SAFE: Errors are logged and rethrown so BullMQ can handle retries and dead-letter logic

import { Worker, Job } from 'bullmq';

const worker = new Worker('email', async (job: Job) => {
  try {
    await sendEmail(job.data.to, job.data.body);
  } catch (err) {
    console.error(`Job ${job.id} failed:`, err);
    throw err;
  }
}, { connection: { host: 'localhost' } });
