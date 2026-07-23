// SAFE: Uses Mongoose with schema validation and explicit query fields
import mongoose from "mongoose";
const UserSchema = new mongoose.Schema({ name: String, role: String, status: String, email: String });
const User = mongoose.model("User", UserSchema);
async function handlerA(req: Request, res: Response) {
    const { role, status } = req.body;
    const query = {};
    if (role) query.role = role;
    if (status) query.status = status;
    const users = await User.find(query).exec();
    res.json(users);
}
async function handlerB(req: Request, res: Response) {
    const sessions = await mongoose.model("Session").find({ role: "admin" }).exec();
    res.json(sessions);
}
