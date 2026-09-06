// [frensense]
// observation: Fixed N+1 query issue by using a batched query outside the loop.

import * as db from 'database'

export async function processItems() {
    const items = await db.getItems(); // Database Source
    
    // Fixed: collect IDs and query once
    const ids = items.map(item => item.id);
    await db.findMany({ where: { id: { $in: ids } } });
}
