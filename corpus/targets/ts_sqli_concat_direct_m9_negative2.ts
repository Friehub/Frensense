// SAFE: Object property value parameterized (alternate)
async function getUserById(req: Request, res: Response) {
    const cfg = { id: req.params.id };
    const result = await db.query("SELECT * FROM users WHERE id = $1", [cfg.id]);
    res.json(result.rows[0]);
}
