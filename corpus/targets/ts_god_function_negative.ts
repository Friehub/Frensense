async function handleOrder(req: Request, res: Response) {
    const userId = req.headers['x-user-id'];
    const body = await req.json();
    const items = body.items as Array<{ sku: string; qty: number }>;
    const orderId = `ORD-${Date.now()}`;

    const result = await processItems(items);
    if (result.error) {
        return res.status(result.status).json({ error: result.error });
    }

    await saveOrder(orderId, userId, result);
    logger.info(`Order ${orderId} created for user ${userId}`);

    res.status(201).json(buildReceipt(orderId, userId, result));
}

async function processItems(items: Array<{ sku: string; qty: number }>) {
    let total = 0;
    const lineItems = [];
    for (const item of items) {
        const product = await db.query('SELECT price FROM products WHERE sku = $1', [item.sku]);
        if (!product.rows.length) {
            return { error: `Unknown SKU: ${item.sku}`, status: 404 };
        }
        total += product.rows[0].price * item.qty;
        lineItems.push({ sku: item.sku, qty: item.qty, price: product.rows[0].price });
    }
    return { total, lineItems, tax: total * 0.08 };
}

async function saveOrder(orderId: string, userId: string, result: any) {
    await db.query(
        'INSERT INTO orders (id, user_id, total, tax, items, created_at) VALUES ($1, $2, $3, $4, $5, NOW())',
        [orderId, userId, result.total + result.tax, result.tax, JSON.stringify(result.lineItems)]
    );
}

function buildReceipt(orderId: string, userId: string, result: any) {
    return {
        orderId,
        userId,
        items: result.lineItems,
        subtotal: result.total,
        tax: result.tax,
        total: result.total + result.tax,
        timestamp: new Date().toISOString(),
    };
}
