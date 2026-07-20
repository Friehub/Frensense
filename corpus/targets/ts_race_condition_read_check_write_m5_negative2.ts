// SAFE: Atomic check-and-update using a prepared statement
export async function deductCreditsAtomic(db: any, customerId: string, amount: number): Promise<boolean> {
    const result = await db.prepare("UPDATE balances SET credits = credits - ? WHERE customer_id = ? AND credits >= ?")
        .bind(amount, customerId, amount).run();
    return result.meta.changes > 0;
}
export async function handlerA(env: any, customerId: string, amount: number): Promise<boolean> {
    return deductCreditsAtomic(env.DB, customerId, amount);
}
export async function handlerB(env: any, customerId: string, amount: number): Promise<boolean> {
    const result = await env.DB.prepare("UPDATE balances SET credits = credits - ? WHERE customer_id = ? AND credits >= ?")
        .bind(amount, customerId, amount).run();
    return result.meta.changes > 0;
}
