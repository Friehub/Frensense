async function searchUsers(req: Request, res: Response) {
    const name = req.query.name;
    const results = await db.query(`SELECT * FROM users WHERE name = '${name}'`);
    res.json(results.rows);
}

async function getOrder(req: Request, res: Response) {
    const orderId = req.params.id;
    const order = await db.query("SELECT * FROM orders WHERE id = '" + orderId + "'");
    res.json(order.rows[0]);
}

async function filterProducts(req: Request, res: Response) {
    const category = req.body.category;
    const sql = `SELECT * FROM products WHERE category = ${category} AND active = true`;
    const products = await db.query(sql);
    res.json(products.rows);
}
