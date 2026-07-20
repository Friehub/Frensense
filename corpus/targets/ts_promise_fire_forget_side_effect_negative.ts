// SAFE: await all side effects or formalize background tasks
async function createOrder(items: any[], db: DB): Promise<void> {
  const order = db.createOrder(items);
  await sendConfirmationEmail(order);
}

async function sendConfirmationEmail(order: Order): Promise<void> {
  const user = await db.findUser(order.userId);
  await emailClient.send(user.email, 'Order confirmed', `Your order #${order.id}`);
}

async function handleRequest(name: string): Promise<string> {
  const id = await db.createRecord(name);
  await logAnalytics('user_created', { name });
  return id;
}

async function logAnalytics(event: string, data: any): Promise<void> {
  await fetch('/analytics', { method: 'POST', body: JSON.stringify(data) });
}
