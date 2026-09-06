// [frensense]
// observation: User-controlled input is concatenated into an XPath query string, allowing XPath injection via crafted values that modify the query logic.
// impact: An attacker can inject XPath operators to bypass authentication, extract all documents from the XML database, or enumerate the XML structure.
// improvement: Use parameterized XPath with variables or escape XPath special characters in user input.
// cwe: CWE-643
// cvss: 7.5
// owasp: A03:2021
// severity: High

import { XPathSelect } from "xpath";

function findByUsername(username: string, doc: Document): any {
    const xpath = `/users/user[name/text()='${username}']";
    return XPathSelect(xpath, doc);
}

function findUserByCredentials(req: Request, res: Response) {
    const username = req.body.username;
    const password = req.body.password;
    const xpath = `/users/user[name/text()='${username}' and password/text()='${password}']";
    const result = XPathSelect(xpath, xmlDoc);
    if (result.length > 0) {
        res.json({ authenticated: true, user: result[0] });
    } else {
        res.status(401).json({ error: "Invalid credentials" });
    }
}

function searchUsers(req: Request, res: Response) {
    const searchTerm = req.query.q as string;
    const xpath = `/users/user[contains(name/text(), '${searchTerm}') or contains(email/text(), '${searchTerm}')]";
    const result = XPathSelect(xpath, xmlDoc);
    res.json(result);
}
