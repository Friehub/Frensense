// SAFE: Stripped newline characters and validated filename against an allowlist pattern.

const SAFE_FILENAME = /^[\w\-\.]+$/;

function sanitizeFilename(name: string): string {
    return name.replace(/[\r\n\0]/g, "").replace(/[^a-zA-Z0-9_\-\.]/g, "_");
}

function downloadFile(req: Request, res: Response) {
    const filename = sanitizeFilename(req.query.file as string);
    if (!SAFE_FILENAME.test(filename)) {
        res.status(400).json({ error: "Invalid filename" });
        return;
    }
    const fileContent = getFileContent(filename);
    res.setHeader("Content-Disposition", `attachment; filename="${filename}"`);
    res.send(fileContent);
}

function exportReport(req: Request, res: Response) {
    const reportName = sanitizeFilename(req.body.reportName);
    const csv = generateCsv(req.body.data);
    res.setHeader("Content-Disposition", `inline; filename="${reportName}.csv"`);
    res.send(csv);
}
