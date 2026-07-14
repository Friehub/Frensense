// [frensense]
// observation: A database record referencing an external resource (like an S3/R2 object) is deleted without first deleting the external resource.
// impact: This causes a storage leak (orphaned files) and potentially violates GDPR if the deleted files contain user data, since the database reference is gone but the file remains forever.
// improvement: Always fetch and delete the associated external resource before deleting the database record.

export async function cleanupOldFiles(db: any, r2: any): Promise<void> {
    // Bad: The database rows are deleted, but the actual files in R2 are left behind
    await db.prepare("DELETE FROM project_files WHERE deleted_at < datetime('now', '-30 days')").run();
}

export async function deleteUserAccount(db: any, s3: any, userId: string): Promise<void> {
    // Bad: The user is deleted from the DB, but their avatar in S3 is orphaned
    await db.prepare("DELETE FROM users WHERE id = ?").bind(userId).run();
}
