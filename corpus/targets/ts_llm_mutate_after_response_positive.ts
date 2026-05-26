function handler(req: any, res: any) {
    res.json({ ok: true });
    db.write("log");
}
