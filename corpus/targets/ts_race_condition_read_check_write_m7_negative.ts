// SAFE: Implements safe alternative
// SAFE: Atomic UPDATE in a single SQL statement prevents double-spend
export async function deductCredits(env: any, customerId: string, amount: number): Promise<boolean> {
    const result = await env.DB.prepare("UPDATE balances SET credits = credits - ? WHERE customer_id = ? AND credits >= ?")
        .bind(amount, customerId, amount).run();
    return result.meta.changes > 0;
}
export async function handlerA(env: any, customerId: string, amount: number): Promise<boolean> {
    return deductCredits(env, customerId, amount);
}
export async function handlerB(env: any, customerId: string, amount: number): Promise<boolean> {
    const result = await env.DB.prepare("UPDATE balances SET credits = credits - ? WHERE customer_id = ? AND credits >= ?")
        .bind(amount, customerId, amount).run();
    return result.meta.changes > 0;
}
