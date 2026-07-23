// [frensense]
// observation: The operator parameter in a Firestore .where() call is taken directly from user input without validation, allowing NoSQL operator injection.
// impact: An attacker can supply operators like array-contains-any, not-in, or custom field paths to bypass security filters and access unauthorized data.
// improvement: Validate the operator against an allowlist of safe operators before calling .where().

import { Firestore } from "@google-cloud/firestore";

const firestore = new Firestore();

async function queryUsers(req: Request, res: Response) {
    const { field, operator, value } = req.body;
    const snapshot = await firestore.collection("users").where(field, operator, value).get();
    const users = snapshot.docs.map(d => ({ id: d.id, ...d.data() }));
    res.json(users);
}

async function messagesByFilter(req: Request, res: Response) {
    const { field, op, val } = req.query;
    const snapshot = await firestore.collection("messages")
        .where(field as string, op as string, val)
        .get();
    const messages = snapshot.docs.map(d => d.data());
    res.json(messages);
}
