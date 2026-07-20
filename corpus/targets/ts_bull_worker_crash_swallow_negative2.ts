// SAFE: No try-catch in the worker, letting errors propagate naturally to BullMQ

import { Worker, Job } from 'bullmq';

const worker = new Worker('email', async (job: Job) => {
  await sendEmail(job.data.to, job.data.body);
}, { connection: { host: 'localhost' } });
