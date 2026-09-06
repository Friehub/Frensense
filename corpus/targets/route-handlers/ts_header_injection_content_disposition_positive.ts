// [frensense]
// observation: User-controlled filename is passed directly to Content-Disposition header, allowing header injection via filename containing newlines or special characters.
// impact: An attacker can inject CRLF sequences into the Content-Disposition header to perform HTTP response splitting, cache poisoning, or XSS.
// improvement: Strip newline characters and validate the filename against a strict allowlist pattern.

function downloadFile(req: Request, res: Response) {
    const filename = req.query.file as string;
    const fileContent = getFileContent(filename);
    res.setHeader("Content-Disposition", `attachment; filename="${filename}"`);
    res.send(fileContent);
}

function exportReport(req: Request, res: Response) {
    const reportName = req.body.reportName;
    const csv = generateCsv(req.body.data);
    res.setHeader("Content-Disposition", `inline; filename="${reportName}.csv"`);
    res.send(csv);
}
