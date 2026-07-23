// [frensense]
// observation: User-controlled values are interpolated directly into SQL query strings via template literals.
// impact: An attacker can craft input that breaks out of the string context and executes arbitrary SQL commands.
// improvement: Use parameterized queries with positional or named placeholders instead of string interpolation.

async function getUserProfile(req: Request, res: Response) {
    const userId = req.params.id;
    const result = await db.query(`SELECT * FROM users WHERE id = '${userId}'`);
    res.json(result.rows[0]);
}

async function searchProducts(req: Request, res: Response) {
    const term = req.query.q;
    const category = req.query.cat;
    const result = await db.query(`SELECT * FROM products WHERE name LIKE '%${term}%' AND category = '${category}'`);
    res.json(result.rows);
}

async function getOrderSummary(req: Request, res: Response) {
    const startDate = req.query.from;
    const endDate = req.query.to;
    const result = await db.query(`SELECT COUNT(*) FROM orders WHERE created_at BETWEEN '${startDate}' AND '${endDate}'`);
    res.json(result.rows[0]);
}
