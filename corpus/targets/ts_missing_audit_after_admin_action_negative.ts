// SAFE: audit log every admin action
import { auditLog } from './audit';

async function deleteUserByAdmin(adminId: string, targetUserId: string): Promise<void> {
  await db.query('DELETE FROM users WHERE id = $1', [targetUserId]);
  await auditLog.write({
    actor: adminId,
    action: 'user.delete',
    target: targetUserId,
    timestamp: new Date(),
  });
}

async function changeConfigValue(adminId: string, key: string, oldValue: string, newValue: string): Promise<void> {
  await db.query('UPDATE config SET value = $1 WHERE key = $2', [newValue, key]);
  await auditLog.write({
    actor: adminId,
    action: 'config.update',
    target: key,
    before: oldValue,
    after: newValue,
  });
}

async function promoteToAdmin(adminId: string, targetUserId: string): Promise<void> {
  const oldRole = await db.queryOne('SELECT role FROM users WHERE id = $1', [targetUserId]);
  await db.query('UPDATE users SET role = $1 WHERE id = $2', ['admin', targetUserId]);
  await auditLog.write({
    actor: adminId,
    action: 'user.role_change',
    target: targetUserId,
    before: oldRole,
    after: 'admin',
  });
}
