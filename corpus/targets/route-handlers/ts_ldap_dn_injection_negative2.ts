// SAFE: Used a DN builder with fixed base components, only accepting the RDN value after validation.

function isValidUid(uid: string): boolean {
    return /^[a-zA-Z0-9_\-\.]+$/.test(uid);
}

function addUser(req: Request, res: Response) {
    const userId = req.body.userId;
    if (!isValidUid(userId)) {
        res.status(400).json({ error: "Invalid user ID" });
        return;
    }
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
    const uid = req.params.id;
    if (!isValidUid(uid)) throw new Error("Invalid UID");
    const dn = `uid=${uid},ou=users,dc=example,dc=com`;
    client.del(dn, err => {
        if (err) return res.status(500).json({ error: err.message });
        res.json({ success: true });
    });
}
