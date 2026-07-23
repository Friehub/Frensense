async function searchUsers(req: Request, res: Response) {
    const name = req.query.name;
    const results = await db.query("SELECT * FROM users WHERE name = $1", [name]);
    res.json(results.rows);
}

async function getOrder(req: Request, res: Response) {
    const orderId = req.params.id;
    const order = await db.query("SELECT * FROM orders WHERE id = $1", [orderId]);
    res.json(order.rows[0]);
}

async function filterProducts(req: Request, res: Response) {
    const category = req.body.category;
    const products = await db.query("SELECT * FROM products WHERE category = $1 AND active = true", [category]);
    res.json(products.rows);
}
