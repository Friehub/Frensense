// SAFE: Validated attribute names against an allowlist before using them in LDAP operations.

const ALLOWED_ATTRIBUTES = new Set([
    "cn", "sn", "mail", "uid", "displayName",
    "telephoneNumber", "title", "department",
]);

function getUserAttribute(req: Request, res: Response) {
    const userId = req.params.id;
    const attrName = req.query.attribute as string;
    if (!ALLOWED_ATTRIBUTES.has(attrName)) {
        res.status(400).json({ error: "Attribute not allowed" });
        return;
    }
    const opts = {
        filter: `(uid=${userId})`,
        scope: "base",
        attributes: [attrName],
    };
    client.search(`uid=${userId},ou=users,dc=example,dc=com`, opts, (err, searchRes) => {
        searchRes.on("searchEntry", entry => {
            res.json({ [attrName]: entry.object[attrName] });
        });
    });
}

function compareAttribute(req: Request, res: Response) {
    const dn = `uid=${req.body.userId},ou=users,dc=example,dc=com`;
    const attr = req.body.attribute;
    const value = req.body.value;
    if (!ALLOWED_ATTRIBUTES.has(attr)) throw new Error("Attribute not allowed");
    client.compare(dn, attr, value, (err, matched) => {
        res.json({ matched });
    });
}
