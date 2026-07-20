// [frensense]
// observation: A catch block re-throws the error without logging or adding any context, destroying information about where and why the error occurred.
// impact: When the error propagates up, the original stack trace and context are lost. Debugging becomes difficult because there is no record of the error at the point it happened. In production, this leads to long incident response times.
// improvement: Log the error with contextual information before re-throwing, or wrap the error with additional context using a custom error type.

async function processOrder(orderId: string) {
  try {
    const order = await db.orders.findUnique({ where: { id: orderId } });
    if (!order) throw new Error('Order not found');
    await chargePayment(order);
    return { success: true };
  } catch (err) {
    throw err;
  }
}

const db = { orders: { findUnique: async (args: any) => args } };
async function chargePayment(order: any) {}
