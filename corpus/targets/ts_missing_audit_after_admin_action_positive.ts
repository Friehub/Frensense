// [frensense]
// observation: Admin action (delete user, change config, modify role) executed without writing an audit log entry.
// impact: Without audit logs, security incidents cannot be investigated. GDPR, SOC2, and PCI-DSS require audit trails for admin actions. Compliance violations can result in fines of up to 4% of global revenue.
// improvement: Write an audit log entry for every admin action: who did what, when, and what the previous value was.

import { db } from './db';

async function deleteUserByAdmin(adminId: string, targetUserId: string): Promise<void> {
  // VULNERABLE: no audit log
  await db.query('DELETE FROM users WHERE id = $1', [targetUserId]);
}

async function changeConfigValue(adminId: string, key: string, newValue: string): Promise<void> {
  // VULNERABLE: no audit log
  await db.query('UPDATE config SET value = $1 WHERE key = $2', [newValue, key]);
}

async function promoteToAdmin(adminId: string, targetUserId: string): Promise<void> {
  // VULNERABLE: role change not logged
  await db.query('UPDATE users SET role = $1 WHERE id = $2', ['admin', targetUserId]);
}
