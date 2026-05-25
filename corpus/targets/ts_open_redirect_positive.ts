// Rule: TS_OPEN_REDIRECT
function handler(req: any, res: any) {
    const url = req.query.url;
    res.redirect(url);
}
