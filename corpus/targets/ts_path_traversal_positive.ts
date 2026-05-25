// Rule: TS_PATH_TRAVERSAL
function readFile(req: any, res: any) {
    const path = req.query.path;
    fs.readFileSync(path);
}
