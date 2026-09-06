// SAFE: Validated the operator against an allowlist of safe Firestore operators before passing to .where().

import { Firestore, WhereFilterOp } from "@google-cloud/firestore";

const firestore = new Firestore();

const ALLOWED_OPERATORS: WhereFilterOp[] = [
    "<", "<=", "==", ">=", ">", "!=",
    "array-contains", "in", "not-in", "array-contains-any",
];

function validateOperator(op: string): WhereFilterOp {
    if (!ALLOWED_OPERATORS.includes(op as WhereFilterOp)) {
        throw new Error(`Invalid operator: ${op}`);
    }
    return op as WhereFilterOp;
}

async function queryUsers(req: Request, res: Response) {
    const { field, operator, value } = req.body;
    const safeOp = validateOperator(operator);
    const snapshot = await firestore.collection("users").where(field, safeOp, value).get();
    const users = snapshot.docs.map(d => ({ id: d.id, ...d.data() }));
    res.json(users);
}

async function messagesByFilter(req: Request, res: Response) {
    const { field, op, val } = req.query;
    const safeOp = validateOperator(op as string);
    const snapshot = await firestore.collection("messages")
        .where(field as string, safeOp, val)
        .get();
    const messages = snapshot.docs.map(d => d.data());
    res.json(messages);
}
