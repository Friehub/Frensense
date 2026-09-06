// SAFE: Escaped XPath special characters in user input before concatenating into the query string.

function escapeXPath(input: string): string {
    return input.replace(/'/g, "&apos;").replace(/"/g, "&quot;").replace(/[\[\]\(\)@]/g, "");
}

function findByUsername(username: string, doc: Document): any {
    const safe = escapeXPath(username);
    const xpath = `/users/user[name/text()='${safe}']";
    return XPathSelect(xpath, doc);
}

function findUserByCredentials(req: Request, res: Response) {
    const username = escapeXPath(req.body.username);
    const password = escapeXPath(req.body.password);
    const xpath = `/users/user[name/text()='${username}' and password/text()='${password}']";
    const result = XPathSelect(xpath, xmlDoc);
    if (result.length > 0) {
        res.json({ authenticated: true, user: result[0] });
    } else {
        res.status(401).json({ error: "Invalid credentials" });
    }
}

function searchUsers(req: Request, res: Response) {
    const searchTerm = escapeXPath(req.query.q as string);
    const xpath = `/users/user[contains(name/text(), '${searchTerm}') or contains(email/text(), '${searchTerm}')]";
    const result = XPathSelect(xpath, xmlDoc);
    res.json(result);
}
