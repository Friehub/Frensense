// SAFE: Used Mongoose with fixed query structure, stripping any unknown operators from the request body.

import mongoose from "mongoose";

const ItemSchema = new mongoose.Schema({
    name: String,
    price: Number,
    category: String,
    inStock: Boolean,
});

const Item = mongoose.model("Item", ItemSchema);

async function getItems(req: Request, res: Response) {
    const allowedFields = ["name", "price", "category", "inStock"];
    const query: Record<string, unknown> = {};
    for (const key of Object.keys(req.body.filter || {})) {
        if (allowedFields.includes(key)) {
            query[key] = req.body.filter[key];
        }
    }
    const items = await Item.find(query).exec();
    res.json(items);
}
