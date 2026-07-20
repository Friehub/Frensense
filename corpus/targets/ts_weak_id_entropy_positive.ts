// [frensense]
// observation: Identifier generated with Date.now() or Math.random() — low entropy, collision-prone under concurrent load.
// impact: Two concurrent registrations within the same millisecond produce identical IDs. Causes silent conflicts, data corruption, or security token guessing.
// improvement: Use crypto.randomUUID() or a cryptographically secure random bytes source for all IDs and tokens.

function createTenant(name: string, ownerId: string) {
  // VULNERABLE: millisecond resolution — collides under concurrent load
  const tenantId = `tnt_${Date.now()}`;
  return { id: tenantId, name, ownerId };
}

function generateSessionToken(): string {
  // VULNERABLE: Math.random is not cryptographically secure
  return Math.random().toString(36).slice(2);
}

function createInviteCode(): string {
  // VULNERABLE: timestamp-based codes are guessable
  return `inv_${new Date().getTime()}`;
}

function generateApiKey(userId: string): string {
  // VULNERABLE: predictable key
  return `key_${userId}_${Date.now()}`;
}
