// [frensense]
// observation: await used inside a .forEach() or .map() callback without Promise.all(). The forEach does not await the returned promises.
// impact: All iterations start concurrently but the function returns before any complete. Side effects (DB writes, API calls) may not finish before the response is sent. Silent data loss.
// improvement: Use a for...of loop with await, or wrap with Promise.all() on an array of promises.

async function processUsers(userIds: string[], db: DB): Promise<void> {
  // VULNERABLE: forEach does not await promises
  userIds.forEach(async (id) => {
    const user = await db.findUser(id);
    await db.sendEmail(user.email, 'Welcome!');
  });
}

async function loadData(ids: number[]): Promise<void> {
  // VULNERABLE: map returns promises, but nothing awaits them
  const results = ids.map(async (id) => {
    const data = await fetch(`/api/item/${id}`);
    return data.json();
  });
  console.log('Done'); // runs before any fetch completes
}
