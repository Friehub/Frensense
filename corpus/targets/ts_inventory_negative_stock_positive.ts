// [frensense]
// observation: Stock is decremented without a guard that prevents it from going below zero, allowing negative inventory levels.
// impact: Negative stock causes systemic accounting problems, allows overselling reports to show fake inventory levels, and prevents accurate reorder calculations.
// improvement: Use an atomic UPDATE with a WHERE stock >= qty guard to prevent the stock from ever going negative.

export async function deductStock(productId: string, quantity: number, env: Env) {
  // VULNERABLE: stock can go negative
  await env.DB.prepare(
    'UPDATE products SET stock = stock - ? WHERE id = ?'
  ).bind(quantity, productId).run();
}
