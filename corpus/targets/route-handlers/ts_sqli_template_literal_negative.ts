// SAFE: Replaced template literal interpolation with parameterized query using $1, $2 positional placeholders.

async function getUserProfile(req: Request, res: Response) {
    const userId = req.params.id;
    const result = await db.query("SELECT * FROM users WHERE id = $1", [userId]);
    res.json(result.rows[0]);
}

async function searchProducts(req: Request, res: Response) {
    const term = req.query.q;
    const category = req.query.cat;
    const result = await db.query("SELECT * FROM products WHERE name LIKE $1 AND category = $2", [`%${term}%`, category]);
    res.json(result.rows);
}

async function getOrderSummary(req: Request, res: Response) {
    const startDate = req.query.from;
    const endDate = req.query.to;
    const result = await db.query("SELECT COUNT(*) FROM orders WHERE created_at BETWEEN $1 AND $2", [startDate, endDate]);
    res.json(result.rows[0]);
}
