// [frensense]
// observation: External resources (like S3/R2 objects) are deleted before their associated database records are removed.
// impact: None — all associated data is properly cleaned up, preventing storage leaks and ensuring compliance.
// improvement: N/A — this is the correct pattern.

export async function cleanupOldFiles(db: any, r2: any): Promise<void> {
    // Good: Fetch expired files, delete from R2, then delete from DB
    const { results: expiredFiles } = await db.prepare(
        "SELECT content FROM project_files WHERE deleted_at < datetime('now', '-30 days') AND content LIKE 'r2://%'"
    ).all();

    for (const f of expiredFiles) {
        const key = f.content.replace("r2://", "");
        await r2.delete(key).catch(() => {});
    }

    await db.prepare("DELETE FROM project_files WHERE deleted_at < datetime('now', '-30 days')").run();
}

export async function deleteUserAccount(db: any, s3: any, userId: string): Promise<void> {
    // Good: Delete avatar from S3 first, then delete user from DB
    const user = await db.prepare("SELECT avatar_key FROM users WHERE id = ?").bind(userId).first();
    if (user && user.avatar_key) {
        await s3.deleteObject({ Bucket: "avatars", Key: user.avatar_key }).promise().catch(() => {});
    }
    await db.prepare("DELETE FROM users WHERE id = ?").bind(userId).run();
}
