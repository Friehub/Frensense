export async function deductCredits(env: any, customerId: string, amount: number): Promise<boolean> {
    const result = await env.DB.prepare(
        "UPDATE balances SET credits = credits - ? WHERE customer_id = ? AND credits >= ?"
    )
    .bind(amount, customerId, amount)
    .run();
    
    return result.meta.changes > 0;
}
