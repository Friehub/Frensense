// SAFE: Renamed variables parameterized (alternate)
async function getUserById(req: Request, res: Response) {
    const userInput = req.params.id;
    const result = await db.query("SELECT * FROM users WHERE id = $1", [userInput]);
    res.json(result.rows[0]);
}
