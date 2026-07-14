// [frensense]
// observation: Balance deduction using an atomic DB transaction that prevents concurrent reads.
// impact: None — the check-and-update is a single atomic SQL operation.
// improvement: N/A — this is the correct pattern.

export async function deductCreditsAtomic(
    db: any,
    customerId: string,
    amount: number
): Promise<boolean> {
    // Atomic: read + check + write in one SQL statement.
    // The WHERE clause prevents double-spend at the database level.
    const result = await db
        .prepare(
            "UPDATE balances SET credits = credits - ? WHERE customer_id = ? AND credits >= ?"
        )
        .bind(amount, customerId, amount)
        .run();
    return result.meta.changes > 0;
}

export async function reserveInventory(
    db: any,
    productId: string,
    qty: number
): Promise<boolean> {
    const result = await db
        .prepare(
            "UPDATE inventory SET reserved = reserved + ? WHERE product_id = ? AND (stock - reserved) >= ?"
        )
        .bind(qty, productId, qty)
        .run();
    return result.meta.changes > 0;
}
