async function runQuery(req: Request, res: Response) {
    const input = req.body.sql;
    const result = await AppDataSource.query("SELECT * FROM users WHERE name = $1", [input]);
    res.json(result);
}
