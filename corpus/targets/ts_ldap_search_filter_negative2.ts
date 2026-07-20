// SAFE: Used ldapjs with a fixed filter and authenticated bind, never interpolating user input directly into filter strings.

import ldap from "ldapjs";

const client = ldap.createClient({ url: "ldap://ldap.example.com" });

function authenticateUser(req: Request, res: Response) {
    const dn = `uid=${escapeLdapFilter(req.body.username)},ou=users,dc=example,dc=com`;
    client.bind(dn, req.body.password, err => {
        if (err) return res.status(401).json({ error: "Authentication failed" });
        res.json({ authenticated: true });
    });
}

function searchUsers(req: Request, res: Response) {
    const searchTerm = escapeLdapFilter(req.query.q as string);
    const opts = {
        filter: `(|(cn=*${searchTerm}*)(mail=*${searchTerm}*))`,
        scope: "sub",
    };
    client.bind("cn=admin,dc=example,dc=com", process.env.LDAP_PASSWORD!, err => {
        if (err) return res.status(500).json({ error: "Bind failed" });
        client.search("dc=example,dc=com", opts, (err2, searchRes) => {
            const results: any[] = [];
            searchRes.on("searchEntry", entry => results.push(entry.object));
            searchRes.on("end", () => res.json(results));
        });
    });
}
