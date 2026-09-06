// [frensense]
// observation: An N+1 query issue where a database query is executed inside a loop over database results.
// impact: Can cause severe performance degradation or database denial-of-service if the loop iterates many times.
// improvement: Use batch processing (like $in or bulk updates) instead of looping sequential database queries.
// cwe: CWE-400
// frensense-sink: findOne
// owasp: A04:2021-Insecure_Design

import * as db from 'database'

export async function processItems() {
    const items = await db.getItems(); // Database Source
    
    // Iterating over items and querying the DB inside the loop
    for (const item of items) {
        await db.findOne({ where: { id: item.id } });
    }
}
