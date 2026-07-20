// SAFE: Job data is parameterized in all SQL queries

import { Worker, Job } from 'bullmq';
import { db } from './database';

const worker = new Worker('order-process', async (job: Job) => {
  const { userId, productId } = job.data;

  const result = await db.query(
    'SELECT * FROM users WHERE id = $1', [userId]
  );

  const product = await db.query(
    'SELECT * FROM products WHERE id = $1', [productId]
  );

  await db.query(
    'INSERT INTO orders (user_id, product_id) VALUES ($1, $2)',
    [userId, productId]
  );
}, { connection: { host: 'localhost' } });
