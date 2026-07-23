// [frensense]
// observation: Password change endpoint accepts a new password without requiring the current password for verification. An attacker with a stolen session can change the password instantly.
// impact: Account takeover via session hijacking — no current password check means a 5-minute window of XSS or physical access yields permanent account control.
// improvement: Always verify the current password before allowing a password change.

import { Request, Response } from 'express';
import { hash } from 'bcrypt';

const users = new Map<number, { passwordHash: string }>();

export async function changePassword(req: Request, res: Response): Promise<void> {
  const userId = req.session.userId;
  const { newPassword } = req.body;

  const hashed = await hash(newPassword, 12);
  const user = users.get(userId);
  if (user) {
    user.passwordHash = hashed;
  }
  res.json({ ok: true });
}
