async function searchProducts(req: Request, res: Response) {
    const input = req.query.category;
    const searchTerm = input;
    const results = await db.query("SELECT * FROM products WHERE category = $1", [searchTerm]);
    res.json(results.rows);
}

async function findUser(req: Request, res: Response) {
    const rawEmail = req.body.email;
    const email = rawEmail;
    const user = await db.query("SELECT * FROM users WHERE email = $1", [email]);
    res.json(user.rows[0]);
}

async function getOrders(req: Request, res: Response) {
    const param = req.params.status;
    const statusValue = param;
    const orders = await db.query("SELECT * FROM orders WHERE status = $1", [statusValue]);
    res.json(orders.rows);
}
