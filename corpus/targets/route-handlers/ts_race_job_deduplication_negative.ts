// SAFE: Each job is deduplicated by storing processed job IDs in the database with a unique constraint before processing
import express from "express";

export async function handleWebhookEvent(req: express.Request, res: express.Response) {
    const event = req.body;
    const dedupKey = `webhook:${event.id}`;
    try {
        await db.query("INSERT INTO processed_jobs (job_id) VALUES (?)", [dedupKey]);
    } catch (err: any) {
        if (err.code === "ER_DUP_ENTRY" || err.code === "SQLITE_CONSTRAINT") {
            return res.json({ received: true, duplicated: true });
        }
        throw err;
    }
    if (event.type === "payment_intent.succeeded") {
        await fulfillOrder(event.data.object.metadata.orderId);
    }
    res.json({ received: true });
}

export async function processQueueJob(job: { id: string; type: string; data: any }) {
    try {
        await db.query("INSERT INTO processed_jobs (job_id) VALUES (?)", [job.id]);
    } catch (err: any) {
        if (err.code === "ER_DUP_ENTRY" || err.code === "SQLITE_CONSTRAINT") {
            return;
        }
        throw err;
    }
    if (job.type === "send_email") {
        await sendEmail(job.data.to, job.data.subject, job.data.body);
    } else if (job.type === "charge_card") {
        await chargeCard(job.data.customerId, job.data.amount);
    }
}
