// [frensense]
// observation: "Worker catch block is empty or only logs, silently swallowing all errors from job processing."
// impact: "Failed jobs are silently consumed as successes, masking bugs, data corruption, and downstream failures. The queue progresses as if nothing went wrong."
// improvement: "Always rethrow or explicitly handle errors in job processors so BullMQ can retry or move jobs to the failed set."

import { Worker, Job } from 'bullmq';

const worker = new Worker('email', async (job: Job) => {
  try {
    await sendEmail(job.data.to, job.data.body);
  } catch (err) {
    console.error('failed');
  }
}, { connection: { host: 'localhost' } });
