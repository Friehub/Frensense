// [frensense]
// observation: Async function that modifies state or performs side effects is called without awaiting its promise (fire-and-forget).
// impact: If the async function throws, the error is an unhandled promise rejection. State updates may be lost. The caller cannot know when the operation completes, leading to race conditions with subsequent requests.
// improvement: Await async side effects, or use structured background job processing with error handling.

function createOrder(items: any[], db: DB): void {
  const order = db.createOrder(items);
  // VULNERABLE: fire-and-forget — errors not caught
  sendConfirmationEmail(order);
}

async function sendConfirmationEmail(order: Order): Promise<void> {
  const user = await db.findUser(order.userId);
  await emailClient.send(user.email, 'Order confirmed', `Your order #${order.id}`);
}

async function handleRequest(name: string): Promise<string> {
  const id = await db.createRecord(name);
  // VULNERABLE: logging side effect not awaited
  logAnalytics('user_created', { name });
  return id;
}

function logAnalytics(event: string, data: any): Promise<void> {
  return fetch('/analytics', { method: 'POST', body: JSON.stringify(data) });
}
