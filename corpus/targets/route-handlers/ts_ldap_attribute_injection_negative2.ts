// SAFE: Used a fixed attribute set in search operations, never accepting attribute names from user input.

function getUserAttribute(req: Request, res: Response) {
    const userId = req.params.id;
    const opts = {
        filter: `(uid=${userId})`,
        scope: "base",
        attributes: ["cn", "sn", "mail", "uid"],
    };
    client.search(`uid=${userId},ou=users,dc=example,dc=com`, opts, (err, searchRes) => {
        searchRes.on("searchEntry", entry => {
            const { cn, sn, mail, uid } = entry.object;
            res.json({ cn, sn, mail, uid });
        });
    });
}

function compareAttribute(req: Request, res: Response) {
    const dn = `uid=${req.body.userId},ou=users,dc=example,dc=com`;
    client.compare(dn, "uid", req.body.userId, (err, matched) => {
        res.json({ matched });
    });
}
