// SAFE: error is wrapped in a custom error class with additional context

class OrderProcessingError extends Error {
  constructor(message: string, public readonly orderId: string, cause: unknown) {
    super(message, { cause });
    this.name = 'OrderProcessingError';
  }
}

async function processOrder(orderId: string) {
  try {
    const order = await db.orders.findUnique({ where: { id: orderId } });
    if (!order) throw new OrderProcessingError('Order not found', orderId, null);
    await chargePayment(order);
    return { success: true };
  } catch (err) {
    if (err instanceof OrderProcessingError) throw err;
    throw new OrderProcessingError('Failed to process order', orderId, err);
  }
}

const db = { orders: { findUnique: async (args: any) => args } };
async function chargePayment(order: any) {}
