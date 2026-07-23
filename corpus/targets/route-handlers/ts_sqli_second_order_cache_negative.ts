async function handleSearch(req: Request, res: Response) {
    const query = req.body.query;
    await redis.set(`search:${req.userId}`, query);
    const cached = await redis.get(`search:${req.userId}`);
    const result = await db.query("SELECT * FROM items WHERE name = $1", [cached]);
    res.json(result.rows);
}
