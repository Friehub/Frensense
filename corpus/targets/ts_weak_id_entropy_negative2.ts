// SAFE: Uses nanoid for compact, collision-resistant, cryptographically secure IDs
import { nanoid } from "nanoid";

function createTenant(name: string, ownerId: string) {
  const tenantId = `tnt_${nanoid(16)}`;
  return { id: tenantId, name, ownerId };
}

function generateSessionToken(): string {
  return nanoid(32);
}

function createInviteCode(): string {
  return `inv_${nanoid(12)}`;
}

function generateApiKey(userId: string): string {
  return `fhp_${userId.slice(0, 8)}_${nanoid(24)}`;
}
