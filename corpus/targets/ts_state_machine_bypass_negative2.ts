// SAFE: Uses a dedicated state machine library (xstate) to enforce valid transitions
import { createMachine, interpret } from "xstate";

const orderMachine = createMachine({
  id: "order",
  initial: "pending",
  states: {
    pending: { on: { CANCEL: "cancelled", PROCESS: "processing" } },
    processing: { on: { CANCEL: "cancelled", SHIP: "shipped" } },
    shipped: { on: { DELIVER: "delivered" } },
    delivered: { on: { REFUND: "refunded" } },
    cancelled: { type: "final" },
    refunded: { type: "final" },
  },
});

async function cancelOrder(orderId: string, db: DB) {
  const order = await db.prepare("SELECT status, payment_id FROM orders WHERE id = ?").bind(orderId).first();
  if (!order) throw new Error("Not found");
  const canCancel = orderMachine.transition(order.status, "CANCEL").changed;
  if (!canCancel) throw new Error("Order cannot be cancelled in its current state");
  await db.prepare('UPDATE orders SET status = "CANCELLED" WHERE id = ? AND status = ?').bind(orderId, order.status).run();
  await issueRefund(order.payment_id);
}
