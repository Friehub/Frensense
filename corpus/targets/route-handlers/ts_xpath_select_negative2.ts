// SAFE: Used an XPath evaluator with variable bindings to prevent injection, avoiding string concatenation entirely.

import { XPathEvaluator } from "xpath";

const evaluator = new XPathEvaluator();

function findUserByCredentials(req: Request, res: Response) {
    const expr = evaluator.createExpression(
        "/users/user[name/text()=$username and password/text()=$password]",
        [["username", "xs:string"], ["password", "xs:string"]],
    );
    const result = expr.evaluate(xmlDoc, null, {
        username: req.body.username,
        password: req.body.password,
    });
    if (result.length > 0) {
        res.json({ authenticated: true, user: result[0] });
    } else {
        res.status(401).json({ error: "Invalid credentials" });
    }
}

function searchUsers(req: Request, res: Response) {
    const expr = evaluator.createExpression(
        "/users/user[contains(name/text(), $search) or contains(email/text(), $search)]",
        [["search", "xs:string"]],
    );
    const result = expr.evaluate(xmlDoc, null, { search: req.query.q });
    res.json(result);
}
