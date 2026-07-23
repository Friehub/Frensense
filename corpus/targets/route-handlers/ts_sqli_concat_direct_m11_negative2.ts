// SAFE: Conditional branch with parameterized query (alternate)
async function getUserById(req: Request, res: Response) {
    if (req.params.id) {
        const result = await db.query("SELECT * FROM users WHERE id = $1", [req.params.id]);
        res.json(result.rows[0]);
    }
}
