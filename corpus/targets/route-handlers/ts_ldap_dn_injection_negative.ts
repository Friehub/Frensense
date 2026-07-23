// SAFE: Escaped DN special characters in each RDN value before constructing the DN string.

function escapeDnValue(input: string): string {
    return input.replace(/[,\\+#<>;"=]/g, "\\$&");
}

function addUser(req: Request, res: Response) {
    const userId = escapeDnValue(req.body.userId);
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
    const uid = escapeDnValue(req.params.id);
    const dn = `uid=${uid},ou=users,dc=example,dc=com`;
    client.del(dn, err => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ success: true });
    });
}
