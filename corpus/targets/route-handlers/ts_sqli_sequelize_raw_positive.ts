async function findUsers(req: Request, res: Response) {
    const name = req.query.name;
    const results = await sequelize.query(`SELECT * FROM users WHERE name LIKE '%${name}%'`);
    res.json(results[0]);
}
