export function extractFileHeader(req: any, res: any, next: any) {
    const fileHeader = req.headers['x-target-file'];
    if (fileHeader) {
        res.locals.targetFile = fileHeader;
    } else {
        res.locals.targetFile = 'default.png';
    }
    next();
}
