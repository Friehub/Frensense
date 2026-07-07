// [frensense]
// observation: The system performs a non-atomic read-check-write sequence to deduct credits or balance. It reads the current balance, checks it in memory, and writes the new balance back.
// impact: In a concurrent environment, two requests can read the same starting balance and pass the check simultaneously, allowing the user to spend more credits than they actually have (double-spend race condition).
// improvement: Use an atomic database update statement (e.g., UPDATE ... SET balance = balance - amount WHERE balance >= amount) and check the rows-changed result.

export async function deductCredits(env: any, customerId: string, amount: number): Promise<boolean> {
    const key = `credits:${customerId}`;
    const raw = await env.KV.get(key);
    const balance = raw ? parseInt(raw, 10) : 0;
    
    if (balance < amount) return false;
    
    await env.KV.put(key, String(balance - amount));
    return true;
}
