async function handleOrder(req: Request, res: Response) {
    const userId = req.headers['x-user-id'];
    const body = await req.json();
    const items = body.items as Array<{ sku: string; qty: number }>;
    const orderId = `ORD-${Date.now()}`;

    let total = 0;
    const lineItems = [];
    for (const item of items) {
        const product = await db.query('SELECT price FROM products WHERE sku = $1', [item.sku]);
        if (!product.rows.length) {
            return res.status(404).json({ error: `Unknown SKU: ${item.sku}` });
        }
        const price = product.rows[0].price * item.qty;
        total += price;
        lineItems.push({ sku: item.sku, qty: item.qty, price });
    }

    const tax = total * 0.08;
    const finalTotal = total + tax;

    await db.query(
        'INSERT INTO orders (id, user_id, total, tax, items, created_at) VALUES ($1, $2, $3, $4, $5, NOW())',
        [orderId, userId, finalTotal, tax, JSON.stringify(lineItems)]
    );

    logger.info(`Order ${orderId} created for user ${userId}, total: ${finalTotal}`);

    const receipt = {
        orderId,
        userId,
        items: lineItems,
        subtotal: total,
        tax,
        total: finalTotal,
        timestamp: new Date().toISOString(),
    };

    res.status(201).json(receipt);
}
