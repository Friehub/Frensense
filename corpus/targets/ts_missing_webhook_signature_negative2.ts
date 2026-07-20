// SAFE: Uses svix library for standard webhook signature verification
import { Webhook } from "svix";

export async function POST(req: Request) {
  const wh = new Webhook(process.env.WEBHOOK_SECRET!);
  const payload = await req.text();
  const signature = req.headers.get("webhook-signature")!;
  try {
    const event = wh.verify(payload, { "webhook-signature": signature });
    if (event.type === "checkout.session.completed") {
      await fulfillOrder(event.data.object.client_reference_id);
      return new Response("Success", { status: 200 });
    }
  } catch {
    return new Response("Bad signature", { status: 401 });
  }
}
