async function buildReport(req: Request, res: Response) {
    const conditions: string[] = [];
    const params: string[] = [];
    const userId = req.body.userId;
    const status = req.body.status;
    conditions.push("user_id = $1");
    params.push(userId);
    conditions.push("status = $2");
    params.push(status);
    const query = "SELECT * FROM reports WHERE " + conditions.join(" AND ");
    const results = await db.query(query, params);
    res.json(results.rows);
}
