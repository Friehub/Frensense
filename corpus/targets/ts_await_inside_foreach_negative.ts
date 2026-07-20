// SAFE: for...of loop preserves sequential await
async function processUsers(userIds: string[], db: DB): Promise<void> {
  for (const id of userIds) {
    const user = await db.findUser(id);
    await db.sendEmail(user.email, 'Welcome!');
  }
}

async function loadData(ids: number[]): Promise<any[]> {
  const results = await Promise.all(ids.map(async (id) => {
    const data = await fetch(`/api/item/${id}`);
    return data.json();
  }));
  return results;
}
