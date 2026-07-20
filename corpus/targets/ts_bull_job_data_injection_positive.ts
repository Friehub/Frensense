// [frensense]
// observation: "Job data from the queue is used directly in SQL queries or file operations without sanitization."
// impact: "An attacker who can enqueue jobs with crafted payloads can perform SQL injection, path traversal, or other injection attacks via job data."
// improvement: "Sanitize and parameterize all queries that use job data, just as you would with HTTP request data."

import { Worker, Job } from 'bullmq';
import { db } from './database';

const worker = new Worker('order-process', async (job: Job) => {
  const { userId, productId } = job.data;

  const result = await db.query(
    `SELECT * FROM users WHERE id = '${userId}'`
  );

  const product = await db.query(
    `SELECT * FROM products WHERE id = '${productId}'`
  );

  await db.query(
    `INSERT INTO orders (user_id, product_id) VALUES ('${userId}', '${productId}')`
  );
}, { connection: { host: 'localhost' } });
