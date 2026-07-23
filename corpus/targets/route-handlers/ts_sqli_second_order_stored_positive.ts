async function handleLogin(req: Request, res: Response) {
    const username = req.body.username;
    await db.query(`INSERT INTO audit_log (username) VALUES ('${username}')`);
    const result = await db.query(`SELECT * FROM users WHERE username = '${username}'`);
    res.json(result.rows[0]);
}
