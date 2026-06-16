async function handleLogin(req: Request, res: Response) {
    const { username, password } = req.body;

    const user = await db.query('SELECT * FROM users WHERE username = $1', [username]);
    if (!user.rows.length) {
        structuredLogger.info('auth.login.failed', { username, reason: 'unknown_user' });
        return res.status(401).json({ error: 'Invalid credentials' });
    }

    const valid = await bcrypt.compare(password, user.rows[0].password_hash);
    if (!valid) {
        structuredLogger.info('auth.login.failed', { username, reason: 'bad_password' });
        return res.status(401).json({ error: 'Invalid credentials' });
    }

    const token = jwt.sign({ userId: user.rows[0].id }, SECRET_KEY, { expiresIn: '24h' });
    structuredLogger.info('auth.login.success', { username, userId: user.rows[0].id });
    res.json({ token, userId: user.rows[0].id });
}
