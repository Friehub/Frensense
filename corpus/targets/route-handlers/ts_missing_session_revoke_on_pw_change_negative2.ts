// SAFE alternative: JWT token blacklist after password change
import { redis } from './redis';
import { signToken } from './jwt';

async function changePassword(userId: string, newPassword: string, db: DB): Promise<void> {
  const hash = await bcrypt.hash(newPassword, 12);
  await db.query('UPDATE users SET password_hash = $1 WHERE id = $2', [hash, userId]);
  // SAFE: add current JWT secret version to user record
  await db.query('UPDATE users SET token_version = token_version + 1 WHERE id = $1', [userId]);
}
// In JWT verification middleware:
function verifyToken(req, res, next) {
  const payload = jwt.verify(req.headers.authorization, process.env.JWT_SECRET);
  const user = await db.queryOne('SELECT token_version FROM users WHERE id = $1', [payload.sub]);
  if (payload.version !== user.token_version) return res.status(401).json({ error: 'Token revoked' });
  next();
}
