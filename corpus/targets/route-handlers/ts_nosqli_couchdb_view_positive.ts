// [frensense]
// observation: User-controlled values are passed directly as startkey/endkey to a CouchDB view query, enabling data extraction beyond the intended range.
// impact: An attacker can enumerate all documents in a database by manipulating key boundaries, or use empty/undefined keys to access restricted data.
// improvement: Validate that the startkey and endkey values match the expected type and are within an acceptable range before querying.
// cwe: CWE-943
// cvss: 8.8
// owasp: A03:2021
// severity: High

import nano from "nano";

const couch = nano(process.env.COUCH_URL!);

async function getUsersByDate(req: Request, res: Response) {
    const db = couch.db.use("users");
    const startKey = req.query.start as string;
    const endKey = req.query.end as string;
    const result = await db.view("users", "by_created", {
        startkey: startKey,
        endkey: endKey,
    });
    res.json(result.rows);
}

async function getOrdersByStatus(req: Request, res: Response) {
    const db = couch.db.use("orders");
    const status = req.query.status as string;
    const result = await db.view("orders", "by_status", {
        startkey: status,
        endkey: status + "\ufff0",
    });
    res.json(result.rows);
}
