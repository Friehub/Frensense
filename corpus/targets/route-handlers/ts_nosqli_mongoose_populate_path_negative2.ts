// SAFE: Used explicit .populate() calls with hardcoded paths instead of accepting paths from request query.

import mongoose from "mongoose";

const OrderSchema = new mongoose.Schema({
    user: { type: mongoose.Schema.Types.ObjectId, ref: "User" },
    product: { type: mongoose.Schema.Types.ObjectId, ref: "Product" },
});

const Order = mongoose.model("Order", OrderSchema);

async function getOrder(req: Request, res: Response) {
    const orderId = req.params.id;
    const order = await Order.findById(orderId)
        .populate("user", "name email")
        .populate("product", "title price")
        .exec();
    res.json(order);
}

async function listOrders(req: Request, res: Response) {
    const orders = await Order.find()
        .populate("user", "name")
        .exec();
    res.json(orders);
}
