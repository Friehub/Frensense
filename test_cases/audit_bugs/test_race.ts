export async function purchaseItem(env: any, userId: string, price: number) {
    const currentStr = await env.KV.get(`user_balance:${userId}`);
    const currentBalance = parseInt(currentStr || "0", 10);
    
    if (currentBalance < price) {
        throw new Error("Not enough balance");
    }
    
    await env.KV.put(`user_balance:${userId}`, (currentBalance - price).toString());
    return true;
}
