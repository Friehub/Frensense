async function handleLogin(req: Request, res: Response) {
    const { username, password } = req.body;

    const user = await db.query('SELECT * FROM users WHERE username = $1', [username]);
    if (!user.rows.length) {
        logger.info(`Login attempt for unknown user: ${username}`);
        return res.status(401).json({ error: 'Invalid credentials' });
    }

    const valid = await bcrypt.compare(password, user.rows[0].password_hash);
    if (!valid) {
        logger.info(`Failed login for user: ${username}`);
        return res.status(401).json({ error: 'Invalid credentials' });
    }

    const token = jwt.sign({ userId: user.rows[0].id }, SECRET_KEY, { expiresIn: '24h' });
    logger.info(`Successful login: ${username}`);
    res.json({ token, userId: user.rows[0].id });
}
