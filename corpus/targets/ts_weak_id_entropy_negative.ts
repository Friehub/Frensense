// SAFE: cryptographically secure ID generation
function createTenant(name: string, ownerId: string) {
  const tenantId = `tnt_${crypto.randomUUID()}`;
  return { id: tenantId, name, ownerId };
}

function generateSessionToken(): string {
  const bytes = new Uint8Array(32);
  crypto.getRandomValues(bytes);
  return Array.from(bytes).map(b => b.toString(16).padStart(2, '0')).join('');
}

function createInviteCode(): string {
  return `inv_${crypto.randomUUID()}`;
}

function generateApiKey(userId: string): string {
  const secret = crypto.randomUUID().replace(/-/g, '');
  return `fhp_${userId.slice(0, 8)}_${secret}`;
}
