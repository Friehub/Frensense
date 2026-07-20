// SAFE: error is logged with contextual information before re-throwing

async function processOrder(orderId: string) {
  try {
    const order = await db.orders.findUnique({ where: { id: orderId } });
    if (!order) throw new Error('Order not found');
    await chargePayment(order);
    return { success: true };
  } catch (err) {
    console.error('Failed to process order', { orderId, error: err });
    throw err;
  }
}

const db = { orders: { findUnique: async (args: any) => args } };
async function chargePayment(order: any) {}
