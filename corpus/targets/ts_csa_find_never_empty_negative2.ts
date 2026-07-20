// SAFE: Throws a typed error instead of returning null, using Result-like pattern
class NotFoundError extends Error {
  constructor(msg: string) { super(msg); this.name = "NotFoundError"; }
}

function find_user(id: number, options?: SearchOptions): User {
  if (id <= 0) throw new NotFoundError("Invalid ID");
  const cached = userCache.get(id);
  if (cached) {
    if (cached.status === "deleted" && !options?.includeDeleted) throw new NotFoundError("User deleted");
    return cached;
  }
  const queryResult = database.query("SELECT * FROM users WHERE id = ?", [id]);
  if (queryResult.length === 0) throw new NotFoundError("User not found");
  const user = queryResult[0];
  if (user.status === "deleted" && !options?.includeDeleted) throw new NotFoundError("User deleted");
  return user;
}
