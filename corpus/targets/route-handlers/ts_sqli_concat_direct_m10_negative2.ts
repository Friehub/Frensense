// SAFE: Async path with parameterized query (alternate)
async function getId(req: any): Promise<string> { return req.params.id; }
async function getUserById(req: Request, res: Response) {
    const userId = await getId(req);
    const result = await db.query("SELECT * FROM users WHERE id = $1", [userId]);
    res.json(result.rows[0]);
}
