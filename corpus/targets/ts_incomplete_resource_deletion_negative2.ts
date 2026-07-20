// SAFE: Uses soft-delete with a background cleanup cron to avoid inline resource management
export async function cleanupOldFiles(db: any, r2: any): Promise<void> {
  await db.prepare("UPDATE project_files SET deleted_at = datetime('now'), marked_for_cleanup = 1 WHERE deleted_at < datetime('now', '-30 days')").run();
}

export async function deleteUserAccount(db: any, s3: any, userId: string): Promise<void> {
  await db.prepare("UPDATE users SET status = 'MARKED_FOR_DELETION' WHERE id = ?").bind(userId).run();
}
