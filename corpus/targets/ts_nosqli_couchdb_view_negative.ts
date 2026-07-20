// SAFE: Validated that the startkey and endkey values are valid ISO date strings before passing to the view query.

import nano from "nano";

const couch = nano(process.env.COUCH_URL!);

const ISO_DATE_REGEX = /^\d{4}-\d{2}-\d{2}(T\d{2}:\d{2}:\d{2}(\.\d+)?Z?)?$/;

function isValidDateKey(key: string): boolean {
    return ISO_DATE_REGEX.test(key);
}

async function getUsersByDate(req: Request, res: Response) {
    const db = couch.db.use("users");
    const startKey = req.query.start as string;
    const endKey = req.query.end as string;
    if (!isValidDateKey(startKey) || !isValidDateKey(endKey)) {
        res.status(400).json({ error: "Invalid date format" });
        return;
    }
    const result = await db.view("users", "by_created", {
        startkey: startKey,
        endkey: endKey,
    });
    res.json(result.rows);
}

async function getOrdersByStatus(req: Request, res: Response) {
    const db = couch.db.use("orders");
    const ALLOWED_STATUSES = ["pending", "confirmed", "shipped", "delivered", "cancelled"];
    const status = req.query.status as string;
    if (!ALLOWED_STATUSES.includes(status)) {
        res.status(400).json({ error: "Invalid status" });
        return;
    }
    const result = await db.view("orders", "by_status", {
        startkey: status,
        endkey: status + "\ufff0",
    });
    res.json(result.rows);
}
