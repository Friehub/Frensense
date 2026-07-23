// SAFE: Used Mongoose with schema validation and explicit query fields instead of raw $where operator.

import mongoose from "mongoose";

const UserSchema = new mongoose.Schema({
    name: String,
    role: String,
    status: String,
    email: String,
});

const User = mongoose.model("User", UserSchema);

async function searchUsers(req: Request, res: Response) {
    const { role, status, email } = req.body;
    const query: Record<string, unknown> = {};
    if (role) query.role = role;
    if (status) query.status = status;
    if (email) query.email = email;
    const users = await User.find(query).exec();
    res.json(users);
}
