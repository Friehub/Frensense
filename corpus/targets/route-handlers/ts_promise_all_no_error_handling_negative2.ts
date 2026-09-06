// SAFE alternative: wrap each promise with error recovery
async function fetchUsers(userIds: string[]): Promise<User[]> {
  const results = await Promise.all(userIds.map(async id => {
    try {
      const res = await fetch(`/api/user/${id}`);
      return await res.json() as User;
    } catch {
      return null;
    }
  }));
  return results.filter(Boolean) as User[];
}
