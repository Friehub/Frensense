// SAFE: Uses pino logger that properly redacts sensitive fields
import pino from "pino";

const logger = pino({
  redact: ["password", "token", "secret"],
});

async function handleLogin(req: Request, res: Response) {
  const { username, password } = req.body;
  const user = await db.query('SELECT * FROM users WHERE username = $1', [username]);
  if (!user.rows.length) {
    logger.warn({ username }, "Login attempt for unknown user");
    return res.status(401).json({ error: 'Invalid credentials' });
  }
  const valid = await bcrypt.compare(password, user.rows[0].password_hash);
  if (!valid) {
    logger.warn({ username }, "Failed login attempt");
    return res.status(401).json({ error: 'Invalid credentials' });
  }
  const token = jwt.sign({ userId: user.rows[0].id }, SECRET_KEY, { expiresIn: '24h' });
  logger.info({ userId: user.rows[0].id }, "Successful login");
  res.json({ token, userId: user.rows[0].id });
}
