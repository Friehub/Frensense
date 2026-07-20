// SAFE: Validated populate paths against an allowlist of permitted relations before passing to .populate().

import mongoose from "mongoose";

const ALLOWED_POPULATE = new Set(["user", "product", "user.profile"]);

const OrderSchema = new mongoose.Schema({
    user: { type: mongoose.Schema.Types.ObjectId, ref: "User" },
    product: { type: mongoose.Schema.Types.ObjectId, ref: "Product" },
});

const Order = mongoose.model("Order", OrderSchema);

function validatePopulatePaths(paths: string[]): string[] {
    return paths.filter(p => ALLOWED_POPULATE.has(p));
}

async function getOrder(req: Request, res: Response) {
    const orderId = req.params.id;
    const populatePath = req.query.include as string;
    const safePath = validatePopulatePaths([populatePath]);
    const order = await Order.findById(orderId).populate(safePath).exec();
    res.json(order);
}

async function listOrders(req: Request, res: Response) {
    const paths = (req.query.populate as string || "").split(",");
    const safePaths = validatePopulatePaths(paths);
    const orders = await Order.find().populate(safePaths).exec();
    res.json(orders);
}
