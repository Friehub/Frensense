// [frensense]
// observation: User-controlled input is used to construct an LDAP distinguished name (DN) for bind/add/delete operations, allowing LDAP injection via crafted DN components.
// impact: An attacker can manipulate the DN structure to bind as a different user, add entries to unauthorized parts of the tree, or delete arbitrary directory entries.
// improvement: Escape DN special characters in each RDN component before constructing the DN string.

import ldap from "ldapjs";

const client = ldap.createClient({ url: "ldap://ldap.example.com" });

function addUser(req: Request, res: Response) {
    const userId = req.body.userId;
    const dn = `uid=${userId},ou=users,dc=example,dc=com`;
    const entry = {
        cn: req.body.fullName,
        sn: req.body.lastName,
        mail: req.body.email,
        objectClass: "inetOrgPerson",
    };
    client.add(dn, entry, err => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ success: true });
    });
}

function deleteUser(req: Request, res: Response) {
    const dn = `uid=${req.params.id},ou=users,dc=example,dc=com`;
    client.del(dn, err => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ success: true });
    });
}
