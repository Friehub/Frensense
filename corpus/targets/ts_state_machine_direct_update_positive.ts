// [frensense]
// observation: The order status is updated directly via a raw SQL UPDATE or ORM update, bypassing the state machine's transition validation — no guard checks whether the transition from current status to new status is valid
// impact: An attacker or buggy client can move an order from any state (e.g., CANCELLED) to any other state (e.g., CONFIRMED), enabling unauthorized fulfillment, double refunds, or inventory manipulation
// improvement: Route all status changes through a state machine that validates transitions and rejects illegal moves, or at minimum add a WHERE clause that confirms the current status allows the intended transition

interface OrderUpdate {
    orderId: string;
    status: string;
}

async function updateOrderStatus(update: OrderUpdate): Promise<void> {
    await db.query(
        `UPDATE orders SET status = '${update.status}' WHERE id = '${update.orderId}'`
    );
}
