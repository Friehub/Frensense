// [frensense]
// observation: User-controlled input is used as an LDAP attribute name in search or compare operations, allowing injection of unintended attribute access.
// impact: An attacker can specify arbitrary attribute names to extract sensitive directory data (e.g., userPassword, ssn) from LDAP entries.
// improvement: Validate attribute names against an allowlist before using them in LDAP operations.

import ldap from "ldapjs";

const client = ldap.createClient({ url: "ldap://ldap.example.com" });

function getUserAttribute(req: Request, res: Response) {
    const userId = req.params.id;
    const attrName = req.query.attribute as string;
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
    client.compare(dn, attr, value, (err, matched) => {
        res.json({ matched });
    });
}
