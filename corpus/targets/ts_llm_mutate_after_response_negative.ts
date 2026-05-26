function handler(req: any, res: any) {
    const data = db.read("config");
    res.json(data);
}
