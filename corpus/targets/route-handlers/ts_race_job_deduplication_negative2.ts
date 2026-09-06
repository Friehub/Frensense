// SAFE: Uses Redis SETNX for distributed job deduplication with a TTL to limit the dedup window
import express from "express";

async function isDuplicate(jobId: string, ttlSeconds: number = 3600): Promise<boolean> {
    const result = await cache.set(jobId, "1", "NX", "EX", ttlSeconds);
    return result !== "OK";
}

export async function handleWebhookEvent(req: express.Request, res: express.Response) {
    const event = req.body;
    const dedupKey = `webhook:${event.id}`;
    if (await isDuplicate(dedupKey)) {
        return res.json({ received: true, duplicated: true });
    }
    if (event.type === "payment_intent.succeeded") {
        await fulfillOrder(event.data.object.metadata.orderId);
    }
    res.json({ received: true });
}

export async function processQueueJob(job: { id: string; type: string; data: any }) {
    if (await isDuplicate(`job:${job.id}`)) return;
    if (job.type === "send_email") {
        await sendEmail(job.data.to, job.data.subject, job.data.body);
    } else if (job.type === "charge_card") {
        await chargeCard(job.data.customerId, job.data.amount);
    }
}
