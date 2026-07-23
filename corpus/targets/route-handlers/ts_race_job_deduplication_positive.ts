// [frensense]
// observation: A queue message/job is processed without deduplication; if the job is delivered twice (at-least-once semantics), it runs twice instead of once.
// impact: Duplicate processing: double-charge, duplicate notification, or duplicate database writes for the same job.
// improvement: Use a deduplication key (job ID or idempotency key) with a unique constraint in the database before processing each job.

import express from "express";

export async function handleWebhookEvent(req: express.Request, res: express.Response) {
    const event = req.body;
    if (event.type === "payment_intent.succeeded") {
        await fulfillOrder(event.data.object.metadata.orderId);
    }
    res.json({ received: true });
}

export async function processQueueJob(job: { id: string; type: string; data: any }) {
    if (job.type === "send_email") {
        await sendEmail(job.data.to, job.data.subject, job.data.body);
    } else if (job.type === "charge_card") {
        await chargeCard(job.data.customerId, job.data.amount);
    }
}
