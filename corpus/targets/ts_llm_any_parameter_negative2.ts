// SAFE: Uses a branded type to prevent raw `any` parameters from passing through

type Sanitized<T> = { readonly __brand: unique symbol } & T;

function sanitizeUser(raw: Record<string, unknown>): Sanitized<UserPayload> {
  if (typeof raw.id !== 'number' || raw.id <= 0) throw new Error('Invalid user ID');
  if (typeof raw.name !== 'string' || raw.name.length === 0) throw new Error('Name is required');
  if (typeof raw.email !== 'string' || !raw.email.includes('@')) throw new Error('Invalid email');
  if (raw.role !== 'admin' && raw.role !== 'user') throw new Error('Invalid role');
  return raw as Sanitized<UserPayload>;
}

function processUser(raw: Sanitized<UserPayload>): UserPayload {
  return { id: raw.id, name: raw.name, email: raw.email, role: raw.role };
}
