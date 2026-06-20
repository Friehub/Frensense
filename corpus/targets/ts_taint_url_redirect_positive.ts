function redirect(req: Request) {
    const url = req.query.next;
    res.redirect(url);
}
