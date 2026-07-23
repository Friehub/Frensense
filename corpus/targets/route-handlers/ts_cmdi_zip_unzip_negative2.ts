// SAFE: Used adm-zip and tar libraries instead of shell commands, avoiding shell injection entirely.

import AdmZip from "adm-zip";
import * as tar from "tar";

function extractArchive(req: Request, res: Response) {
    const archivePath = "/uploads/" + req.body.archive;
    const destDir = "/extract/" + req.body.destination;
    const zip = new AdmZip(archivePath);
    zip.extractAllTo(destDir, true);
    res.json({ message: "Extracted successfully" });
}

function compressFiles(req: Request, res: Response) {
    const output = "/uploads/" + req.body.output;
    const zip = new AdmZip();
    for (const file of req.body.files) {
        zip.addLocalFile("/uploads/" + file);
    }
    zip.writeZip(output);
    res.json({ message: "Compressed successfully" });
}

function untarArchive(req: Request, res: Response) {
    const archive = req.query.file as string;
    await tar.extract({
        file: "/uploads/" + archive,
        cwd: "/tmp/extract",
    });
    res.json({ message: "Extracted successfully" });
}
