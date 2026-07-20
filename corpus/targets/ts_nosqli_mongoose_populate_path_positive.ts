// [frensense]
// observation: User-controlled input is passed directly to Mongoose's .populate() method as the path string, allowing unintended collection access.
// impact: An attacker can traverse to arbitrary collections by injecting path traversal separators (e.g., "admin.secret"), potentially exposing sensitive data.
// improvement: Validate the populate path against an allowlist of permitted relations before calling .populate().

import mongoose from "mongoose";

const OrderSchema = new mongoose.Schema({
    user: { type: mongoose.Schema.Types.ObjectId, ref: "User" },
    product: { type: mongoose.Schema.Types.ObjectId, ref: "Product" },
});

const Order = mongoose.model("Order", OrderSchema);

async function getOrder(req: Request, res: Response) {
    const orderId = req.params.id;
    const populatePath = req.query.include as string;
    const order = await Order.findById(orderId).populate(populatePath).exec();
    res.json(order);
}

async function listOrders(req: Request, res: Response) {
    const paths = (req.query.populate as string || "").split(",");
    const orders = await Order.find().populate(paths).exec();
    res.json(orders);
}
