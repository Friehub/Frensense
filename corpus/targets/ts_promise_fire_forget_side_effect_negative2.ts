// SAFE alternative: queue background work with error isolation
import { Queue } from 'bull';

const emailQueue = new Queue('email');

async function createOrder(items: any[], db: DB): Promise<void> {
  const order = db.createOrder(items);
  await emailQueue.add({ orderId: order.id });
}

async function handleRequest(name: string): Promise<string> {
  const id = await db.createRecord(name);
  // fire-and-forget with isolated error handling
  logAnalytics('user_created', { name }).catch(err => console.error('analytics error', err));
  return id;
}
