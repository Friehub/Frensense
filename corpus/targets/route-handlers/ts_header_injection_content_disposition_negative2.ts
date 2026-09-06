// SAFE: Used a fixed, hardcoded filename prefix with only an index from user input, preventing header injection.

function downloadFile(req: Request, res: Response) {
    const fileId = req.query.fileId as string;
    const safeId = fileId.replace(/[^0-9a-f\-]/g, "");
    const filename = `export-${safeId}.pdf`;
    const fileContent = getFileContent(safeId);
    res.setHeader("Content-Disposition", `attachment; filename="${filename}"`);
    res.send(fileContent);
}

function exportReport(req: Request, res: Response) {
    const timestamp = Date.now();
    const filename = `report-${timestamp}.csv`;
    const csv = generateCsv(req.body.data);
    res.setHeader("Content-Disposition", `inline; filename="${filename}"`);
    res.send(csv);
}
