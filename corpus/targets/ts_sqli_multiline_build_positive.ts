async function buildReport(req: Request, res: Response) {
    const filters: string[] = [];
    const userId = req.body.userId;
    const status = req.body.status;
    filters.push(`user_id = '${userId}'`);
    filters.push(`status = '${status}'`);
    const query = "SELECT * FROM reports WHERE " + filters.join(" AND ");
    const results = await db.query(query);
    res.json(results.rows);
}
