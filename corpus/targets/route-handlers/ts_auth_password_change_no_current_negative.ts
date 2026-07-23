// SAFE: Current password is verified before allowing a password change.
import { Request, Response } from 'express';
import { hash, compare } from 'bcrypt';

const users = new Map<number, { passwordHash: string }>();

export async function changePassword(req: Request, res: Response): Promise<void> {
  const userId = req.session.userId;
  const { currentPassword, newPassword } = req.body;

  if (!currentPassword || !newPassword) {
    res.status(400).json({ error: 'currentPassword and newPassword required' });
    return;
  }

  const user = users.get(userId);
  if (!user) {
    res.status(404).json({ error: 'user not found' });
    return;
  }

  const valid = await compare(currentPassword, user.passwordHash);
  if (!valid) {
    res.status(403).json({ error: 'current password is incorrect' });
    return;
  }

  user.passwordHash = await hash(newPassword, 12);
  res.json({ ok: true });
}
