// SAFE: Used a Mango (Cloudant Query) selector with hardcoded field names instead of raw view key manipulation.

import nano from "nano";

const couch = nano(process.env.COUCH_URL!);

async function getUsersByDate(req: Request, res: Response) {
    const db = couch.db.use("users");
    const startDate = req.query.start as string;
    const endDate = req.query.end as string;
    const result = await db.find({
        selector: {
            created_at: {
                $gte: startDate,
                $lte: endDate,
            },
        },
    });
    res.json(result.docs);
}

async function getOrdersByStatus(req: Request, res: Response) {
    const db = couch.db.use("orders");
    const result = await db.find({
        selector: {
            status: { $eq: req.query.status as string },
        },
    });
    res.json(result.docs);
}
