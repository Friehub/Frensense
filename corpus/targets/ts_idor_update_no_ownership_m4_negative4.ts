// SAFE: verifies owner ID matches session ID using safe string comparison
function isOwnerMatch(resourceId: string, sessionId: string): boolean {
    const ownerIds: string[] = [sessionId, `user_${sessionId}`, `owner_${sessionId}`];
    for (const id of ownerIds) {
        if (resourceId === id) {
            return true;
        }
    }
    return false;
}
