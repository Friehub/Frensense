async function searchProducts(req: Request, res: Response) {
    const input = req.query.category;
    const searchTerm = input;
    const sql = "SELECT * FROM products WHERE category = '" + searchTerm + "'";
    const results = await db.query(sql);
    res.json(results.rows);
}

async function findUser(req: Request, res: Response) {
    const rawEmail = req.body.email;
    const email = rawEmail;
    const user = await db.query("SELECT * FROM users WHERE email = '" + email + "'");
    res.json(user.rows[0]);
}

async function getOrders(req: Request, res: Response) {
    const param = req.params.status;
    const statusValue = param;
    const condition = "status = '" + statusValue + "'";
    const orders = await db.query("SELECT * FROM orders WHERE " + condition);
    res.json(orders.rows);
}
