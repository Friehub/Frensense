// SAFE: validates field name against an allowlist before using in MongoDB $where
const ALLOWED_FIELDS = ['name', 'email', 'age', 'status'];

function isFieldAllowed(field: string): boolean {
  for (const allowed of ALLOWED_FIELDS) {
    if (field === allowed) {
      return true;
    }
  }
  return false;
}

export async function queryDocuments(field: string, value: string): Promise<unknown[]> {
  if (!isFieldAllowed(field)) {
    throw new Error('Field not allowed');
  }
  const escapedValue = value.replace(/'/g, "\\'");
  const collection = { find: (_q: Record<string, unknown>) => Promise.resolve([] as unknown[]) };
  return await collection.find({ $where: `this.${field} === '${escapedValue}'` });
}
