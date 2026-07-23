// SAFE: Escaped LDAP special characters in user input before constructing the search filter.

function escapeLdapFilter(input: string): string {
    return input.replace(/[\\\*\(\)\0\/]/g, "\\$&");
}

function authenticateUser(req: Request, res: Response) {
    const username = escapeLdapFilter(req.body.username);
    const opts = {
        filter: `(uid=${username})`,
        scope: "sub",
    };
    client.search("ou=users,dc=example,dc=com", opts, (err, searchRes) => {
        if (err) return res.status(500).json({ error: err.message });
        searchRes.on("searchEntry", entry => {
            const valid = entry.object.userPassword === req.body.password;
            res.json({ authenticated: valid, user: entry.object });
        });
    });
}

function searchUsers(req: Request, res: Response) {
    const searchTerm = escapeLdapFilter(req.query.q as string);
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
