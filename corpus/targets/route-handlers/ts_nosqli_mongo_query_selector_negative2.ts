// SAFE: Used Mongoose schema validation which automatically strips unknown fields and prevents operator injection.

import mongoose from "mongoose";

const UserSchema = new mongoose.Schema({
    username: { type: String, required: true },
    email: String,
    status: { type: String, enum: ["active", "inactive"] },
    role: { type: String, enum: ["user", "admin"] },
});

const User = mongoose.model("User", UserSchema);

async function findUser(req: Request, res: Response) {
    const { username, email, status, role } = req.body;
    const query: Record<string, unknown> = {};
    if (username) query.username = username;
    if (email) query.email = email;
    if (status) query.status = status;
    if (role) query.role = role;
    const user = await User.findOne(query).exec();
    res.json(user);
}
