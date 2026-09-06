// SAFE: Hardcoded the query field and operator, only accepting the value from user input — preventing operator injection.

import { Firestore } from "@google-cloud/firestore";

const firestore = new Firestore();

async function queryUsers(req: Request, res: Response) {
    const { field } = req.body;
    const ALLOWED_FIELDS = ["email", "username", "role", "status"];
    if (!ALLOWED_FIELDS.includes(field)) {
        res.status(400).json({ error: "Invalid field" });
        return;
    }
    const snapshot = await firestore.collection("users").where(field, "==", req.body.value).get();
    const users = snapshot.docs.map(d => ({ id: d.id, ...d.data() }));
    res.json(users);
}

async function messagesByFilter(req: Request, res: Response) {
    const snapshot = await firestore.collection("messages")
        .where("senderId", "==", req.query.senderId)
        .get();
    const messages = snapshot.docs.map(d => d.data());
    res.json(messages);
}
