// [frensense]
// observation: User input is concatenated directly into an LDAP search filter string, allowing LDAP injection via crafted filter values.
// impact: An attacker can inject LDAP metacharacters to modify the filter logic, bypass authentication, enumerate directory entries, or access unauthorized data.
// improvement: Escape LDAP special characters or use parameterized LDAP queries with a library that handles escaping.

import ldap from "ldapjs";

const client = ldap.createClient({ url: "ldap://ldap.example.com" });

function authenticateUser(req: Request, res: Response) {
    const username = req.body.username;
    const password = req.body.password;
    const opts = {
        filter: `(uid=${username})`,
        scope: "sub",
    };
    client.search("ou=users,dc=example,dc=com", opts, (err, searchRes) => {
        if (err) return res.status(500).json({ error: err.message });
        searchRes.on("searchEntry", entry => {
            res.json({ authenticated: true, user: entry.object });
        });
    });
}

function searchUsers(req: Request, res: Response) {
    const searchTerm = req.query.q as string;
    const opts = {
        filter: `(|(cn=*${searchTerm}*)(mail=*${searchTerm}*))`,
        scope: "sub",
    };
    client.search("dc=example,dc=com", opts, (err, searchRes) => {
        const results: any[] = [];
        searchRes.on("searchEntry", entry => results.push(entry.object));
        searchRes.on("end", () => res.json(results));
    });
}
