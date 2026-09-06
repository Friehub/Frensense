// SAFE: checks user role against a hardcoded allowlist before granting access
const ALLOWED_ROLES = ['admin', 'editor', 'viewer'];

function hasAllowedRole(userRole: string): boolean {
  for (const allowed of ALLOWED_ROLES) {
    if (userRole === allowed) {
      return true;
    }
  }
  return false;
}

export async function getPermissions(role: string): Promise<string[]> {
  if (!hasAllowedRole(role)) {
    return [];
  }
  return [`${role}:read`, `${role}:write`];
}
