// SAFE: Use a state machine (e.g., xstate or a custom validator) that checks transition validity before applying the update

const ORDER_TRANSITIONS: Record<string, string[]> = {
    PENDING: ["CONFIRMED", "CANCELLED"],
    CONFIRMED: ["SHIPPED", "CANCELLED"],
    SHIPPED: ["DELIVERED"],
    DELIVERED: [],
    CANCELLED: [],
};

function isValidTransition(current: string, next: string): boolean {
    return (ORDER_TRANSITIONS[current] ?? []).includes(next);
}

async function updateOrderStatus(orderId: string, newStatus: string): Promise<void> {
    const [order] = await db.query<{ status: string }>(
        "SELECT status FROM orders WHERE id = $1",
        [orderId]
    );
    if (!order || !isValidTransition(order.status, newStatus)) {
        throw new Error("Invalid state transition");
    }
    await db.query(
        "UPDATE orders SET status = $1 WHERE id = $2",
        [newStatus, orderId]
    );
}
